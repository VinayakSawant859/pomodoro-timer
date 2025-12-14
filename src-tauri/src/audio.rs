use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
use std::io::Cursor;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager};

/// Audio state that holds the persistent audio stream handle and background sink
/// 
/// The OutputStream must be kept alive for the duration of the app, but it's not Send.
/// We keep it in a separate struct that's managed by the main thread.
pub struct AudioState {
    /// The stream handle can be safely shared and used to create sinks
    pub handle: OutputStreamHandle,
    /// Background sink for looping ambient sounds (white noise, rain, etc.)
    pub bg_sink: Arc<Mutex<Option<Sink>>>,
}

impl AudioState {
    /// Create a new AudioState with a stream handle
    /// Note: The OutputStream must be kept alive by the caller
    pub fn new(handle: OutputStreamHandle) -> Self {
        Self {
            handle,
            bg_sink: Arc::new(Mutex::new(None)),
        }
    }
}

/// Container for the OutputStream that must stay alive
/// This is not Send, so it must be kept in the main thread
pub struct AudioStream {
    #[allow(dead_code)]
    stream: OutputStream,
}

impl AudioStream {
    pub fn new() -> Result<(Self, OutputStreamHandle), String> {
        let (stream, handle) = OutputStream::try_default()
            .map_err(|e| format!("Failed to initialize audio output: {}", e))?;
        
        Ok((AudioStream { stream }, handle))
    }
}

/// Play a one-shot sound effect (fire and forget)
#[tauri::command]
pub async fn play_sound(
    state: tauri::State<'_, AudioState>,
    sound_name: String,
    app_handle: AppHandle,
) -> Result<(), String> {
    let handle = state.handle.clone();
    
    tokio::spawn(async move {
        if let Err(e) = play_sound_impl(handle, sound_name, app_handle).await {
            eprintln!("Failed to play sound: {}", e);
        }
    });
    
    Ok(())
}

/// Internal implementation of sound playing
async fn play_sound_impl(
    stream_handle: OutputStreamHandle,
    sound_name: String,
    app_handle: AppHandle,
) -> Result<(), String> {
    let sink = Sink::try_new(&stream_handle)
        .map_err(|e| format!("Failed to create audio sink: {}", e))?;
    
    let file_name = if sound_name.contains('.') {
        sound_name.clone()
    } else {
        format!("{}.wav", sound_name)
    };
    
    let audio_path = find_audio_file(&file_name, &app_handle)?;
    
    let file = std::fs::File::open(&audio_path)
        .map_err(|e| format!("Failed to open audio file: {}", e))?;
    
    let source = Decoder::new(std::io::BufReader::new(file))
        .map_err(|e| format!("Failed to decode audio: {}", e))?;
    
    sink.append(source);
    sink.sleep_until_end();
    
    Ok(())
}

/// Set or stop background white noise/ambient sound
/// 
/// If sound_name is Some("rain"), it loads rain.wav and loops it indefinitely
/// If sound_name is None, it stops the current background sound
#[tauri::command]
pub async fn set_white_noise(
    state: tauri::State<'_, AudioState>,
    sound_name: Option<String>,
    app_handle: AppHandle,
) -> Result<(), String> {
    let mut bg_sink_guard = state.bg_sink.lock()
        .map_err(|e| format!("Failed to acquire background sink lock: {}", e))?;
    
    if let Some(existing_sink) = bg_sink_guard.take() {
        existing_sink.stop();
    }
    
    if let Some(sound) = sound_name {
        let file_name = if sound.contains('.') {
            sound.clone()
        } else {
            format!("{}.wav", sound)
        };
        
        let audio_path = find_audio_file(&file_name, &app_handle)?;
        
        let sink = Sink::try_new(&state.handle)
            .map_err(|e| format!("Failed to create background sink: {}", e))?;
        
        let file = std::fs::File::open(&audio_path)
            .map_err(|e| format!("Failed to open audio file: {}", e))?;
        
        let source = Decoder::new(std::io::BufReader::new(file))
            .map_err(|e| format!("Failed to decode audio: {}", e))?;
        
        let looped_source = source.repeat_infinite();
        sink.append(looped_source);
        
        // Set moderate volume for ambient noise (0.50 = 50% volume)
        // This ensures it's audible but not overpowering
        sink.set_volume(0.70);
        *bg_sink_guard = Some(sink);
    }
    
    Ok(())
}

/// Get the current volume of the background sound
#[tauri::command]
pub fn get_white_noise_volume(state: tauri::State<'_, AudioState>) -> Result<f32, String> {
    let bg_sink_guard = state.bg_sink.lock()
        .map_err(|e| format!("Failed to acquire background sink lock: {}", e))?;
    
    if let Some(sink) = bg_sink_guard.as_ref() {
        Ok(sink.volume())
    } else {
        Ok(0.0)
    }
}

/// Set the volume of the background white noise (0.0 to 1.0)
#[tauri::command]
pub fn set_white_noise_volume(
    state: tauri::State<'_, AudioState>,
    volume: f32,
) -> Result<(), String> {
    let bg_sink_guard = state.bg_sink.lock()
        .map_err(|e| format!("Failed to acquire background sink lock: {}", e))?;
    
    if let Some(sink) = bg_sink_guard.as_ref() {
        let clamped_volume = volume.clamp(0.0, 1.0);
        sink.set_volume(clamped_volume);
    }
    
    Ok(())
}

/// Check if white noise is currently playing
#[tauri::command]
pub fn is_white_noise_playing(state: tauri::State<'_, AudioState>) -> Result<bool, String> {
    let bg_sink_guard = state.bg_sink.lock()
        .map_err(|e| format!("Failed to acquire background sink lock: {}", e))?;
    
    Ok(bg_sink_guard.is_some())
}

/// Helper function to find audio files in various possible locations
fn find_audio_file(file_name: &str, app_handle: &AppHandle) -> Result<PathBuf, String> {
    if let Ok(resource_path) = app_handle.path().resource_dir() {
        let path = resource_path.join(file_name);
        println!("Trying resource path: {:?}, exists: {}", path, path.exists());
        if path.exists() {
            return Ok(path);
        }
        
        let path = resource_path.join("_up_").join("static").join(file_name);
        println!("Trying resource _up_/static path: {:?}, exists: {}", path, path.exists());
        if path.exists() {
            return Ok(path);
        }
    }
    
    if let Ok(current_dir) = std::env::current_dir() {
        let path = current_dir.join("..").join("static").join(file_name);
        println!("Trying dev path: {:?}, exists: {}", path, path.exists());
        if path.exists() {
            return Ok(path);
        }
    }
    
    let path = PathBuf::from(file_name);
    if path.exists() {
        return Ok(path);
    }
    
    Err(format!("Audio file not found: {}", file_name))
}

/// Legacy command for backward compatibility
#[tauri::command]
pub async fn play_notification_sound(
    state: tauri::State<'_, AudioState>,
    sound_type: String,
) -> Result<(), String> {
    let handle = state.handle.clone();
    
    tokio::spawn(async move {
        if let Err(e) = play_notification_impl(handle, sound_type).await {
            eprintln!("Failed to play notification sound: {}", e);
        }
    });
    
    Ok(())
}

/// Internal implementation for notification sounds
async fn play_notification_impl(
    stream_handle: OutputStreamHandle,
    sound_type: String,
) -> Result<(), String> {
    let sink = Sink::try_new(&stream_handle)
        .map_err(|e| format!("Failed to create audio sink: {}", e))?;
    
    // Generate simple beep sounds (placeholder implementation)
    let sound_data = match sound_type.as_str() {
        "work_complete" => generate_work_complete_sound(),
        "break_complete" => generate_break_complete_sound(),
        "tick" => generate_tick_sound(),
        _ => generate_tick_sound(),
    };
    
    let cursor = Cursor::new(sound_data);
    let source = Decoder::new(cursor)
        .map_err(|e| format!("Failed to decode generated sound: {}", e))?;
    
    sink.append(source);
    sink.sleep_until_end();
    
    Ok(())
}

fn generate_work_complete_sound() -> Vec<u8> {
    vec![0; 1000] // Placeholder empty audio data
}

fn generate_break_complete_sound() -> Vec<u8> {
    vec![0; 1000] // Placeholder empty audio data
}

fn generate_tick_sound() -> Vec<u8> {
    vec![0; 100] // Placeholder empty audio data
}
