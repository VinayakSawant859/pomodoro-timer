<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    let selectedNoise = $state("boiler-ambient-noise.mp3");
    let isPlaying = $state(false);
    let isBrowsing = $state(false);
    let coverError = $state(false);

    // Short, calm labels for Zen Mode
    const whiteNoiseOptions = [
        {
            name: "Boiler",
            fullName: "Boiler Ambient Noise",
            file: "boiler-ambient-noise.mp3",
        },
        { name: "Ambi", fullName: "Ambi Val", file: "Ambi-Val.mp3" },
        {
            name: "Clouds",
            fullName: "Cumulus Clouds",
            file: "Cumulus-Clouds.mp3",
        },
        {
            name: "Nature",
            fullName: "Natural Sample",
            file: "Natural-Sample-Makers.mp3",
        },
        {
            name: "Quantum",
            fullName: "Quantum White",
            file: "Quantum-White.mp3",
        },
        {
            name: "Tranquil",
            fullName: "Tranquil White Noise",
            file: "Tranquil-White-Noise.mp3",
        },
    ];

    // Get current track info
    const currentTrack = $derived(
        whiteNoiseOptions.find((o) => o.file === selectedNoise) ||
            whiteNoiseOptions[0],
    );

    // Derive cover image path from audio filename
    const coverImagePath = $derived(
        `/white-noise/${selectedNoise.replace(".mp3", ".jpg")}`,
    );

    // Get cover path for any track option
    function getOptionCoverPath(file: string): string {
        return `/white-noise/${file.replace(".mp3", ".jpg")}`;
    }

    // Reset cover error when track changes
    $effect(() => {
        selectedNoise;
        coverError = false;
    });

    async function playWhiteNoise(fileName: string) {
        try {
            await invoke("set_white_noise", {
                soundName: `white-noise/${fileName}`,
            });
            selectedNoise = fileName;
            isPlaying = true;
            localStorage.setItem("selectedWhiteNoise", fileName);
            localStorage.setItem("ambientNoiseEnabled", "true");
        } catch (error) {
            console.error("Failed to play white noise:", error);
        }
    }

    async function stopWhiteNoise() {
        try {
            await invoke("set_white_noise", { soundName: null });
            isPlaying = false;
            localStorage.setItem("ambientNoiseEnabled", "false");
        } catch (error) {
            console.error("Failed to stop white noise:", error);
        }
    }

    async function toggleWhiteNoise() {
        if (isPlaying) {
            await stopWhiteNoise();
        } else {
            await playWhiteNoise(selectedNoise);
        }
    }

    function selectSound(fileName: string) {
        playWhiteNoise(fileName);
        // Close browse panel after selection with slight delay for smooth UX
        setTimeout(() => {
            isBrowsing = false;
        }, 180);
    }

    function toggleBrowse() {
        isBrowsing = !isBrowsing;
    }

    function handleCoverError() {
        coverError = true;
    }

    onMount(() => {
        const savedNoise = localStorage.getItem("selectedWhiteNoise");
        const savedEnabled = localStorage.getItem("ambientNoiseEnabled");

        if (savedNoise) {
            selectedNoise = savedNoise;
        }

        if (savedEnabled === "true") {
            isPlaying = true;
            playWhiteNoise(selectedNoise);
        }
    });
</script>

<div class="ambient-card" class:browsing={isBrowsing}>
    <!-- Header with title and Browse button -->
    <div class="ambient-header">
        <span class="ambient-title">Ambient Sounds</span>
        <button
            class="browse-btn"
            class:active={isBrowsing}
            onclick={toggleBrowse}
        >
            {isBrowsing ? "Close" : "Browse"}
        </button>
    </div>

    <!-- Content area with Now Playing and Track Grid -->
    <div class="ambient-content">
        <!-- Now Playing Inner Card -->
        <div class="now-playing-card" class:compact={isBrowsing}>
            <div class="now-playing-label">
                Now Playing: {currentTrack.name}
            </div>

            <div class="player-content">
                <!-- Track Cover with Image Support -->
                <div class="track-cover" class:playing={isPlaying}>
                    {#if !coverError}
                        <img
                            src={coverImagePath}
                            alt={currentTrack.name}
                            class="cover-image"
                            onerror={handleCoverError}
                        />
                        <div class="cover-overlay"></div>
                    {:else}
                        <div class="cover-icon">
                            <svg
                                viewBox="0 0 24 24"
                                fill="none"
                                stroke="currentColor"
                            >
                                <path
                                    d="M9 18V5l12-2v13M9 18c0 1.657-1.343 3-3 3s-3-1.343-3-3 1.343-3 3-3 3 1.343 3 3zm12-3c0 1.657-1.343 3-3 3s-3-1.343-3-3 1.343-3 3-3 3 1.343 3 3z"
                                ></path>
                            </svg>
                        </div>
                    {/if}
                </div>

                <!-- Beat Visualizer (7 bars, atmospheric) -->
                <div
                    class="visualizer"
                    class:playing={isPlaying}
                    class:hidden={isBrowsing}
                >
                    <div class="bar"></div>
                    <div class="bar"></div>
                    <div class="bar"></div>
                    <div class="bar"></div>
                    <div class="bar"></div>
                    <div class="bar"></div>
                    <div class="bar"></div>
                </div>

                <!-- Play/Pause Button -->
                <button
                    class="play-pause-btn"
                    class:playing={isPlaying}
                    onclick={toggleWhiteNoise}
                    title={isPlaying ? "Pause" : "Play"}
                >
                    {#if isPlaying}
                        <svg viewBox="0 0 24 24" fill="currentColor">
                            <rect x="6" y="4" width="4" height="16" rx="1"
                            ></rect>
                            <rect x="14" y="4" width="4" height="16" rx="1"
                            ></rect>
                        </svg>
                    {:else}
                        <svg viewBox="0 0 24 24" fill="currentColor">
                            <polygon points="6,4 20,12 6,20"></polygon>
                        </svg>
                    {/if}
                </button>
            </div>
        </div>

        <!-- Track Selection Grid (visible when browsing) -->
        <div class="track-grid" class:visible={isBrowsing}>
            {#each whiteNoiseOptions as option}
                <button
                    class="track-option"
                    class:selected={selectedNoise === option.file}
                    onclick={() => selectSound(option.file)}
                    title={option.fullName}
                >
                    <div class="option-cover">
                        <img
                            src={getOptionCoverPath(option.file)}
                            alt={option.name}
                            class="option-cover-image"
                            onerror={(e) =>
                                (e.currentTarget.style.display = "none")}
                        />
                        <svg
                            class="option-cover-fallback"
                            viewBox="0 0 24 24"
                            fill="none"
                            stroke="currentColor"
                        >
                            <path
                                d="M9 18V5l12-2v13M9 18c0 1.657-1.343 3-3 3s-3-1.343-3-3 1.343-3 3-3 3 1.343 3 3zm12-3c0 1.657-1.343 3-3 3s-3-1.343-3-3 1.343-3 3-3 3 1.343 3 3z"
                            ></path>
                        </svg>
                    </div>
                    <span class="option-name">{option.name}</span>
                </button>
            {/each}
        </div>
    </div>
</div>

<style>
    .ambient-card {
        background: var(--surface-color);
        border-radius: 1.25rem;
        padding: 1.25rem;
        box-shadow:
            0 1px 3px rgba(0, 0, 0, 0.08),
            0 4px 12px rgba(0, 0, 0, 0.05),
            0 0 0 1px var(--border-color);
        width: 100%;
        transition: all 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    }

    /* Header */
    .ambient-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        margin-bottom: 1rem;
    }

    .ambient-title {
        font-size: 0.875rem;
        font-weight: 600;
        color: var(--text-color);
    }

    .browse-btn {
        padding: 0.375rem 0.875rem;
        border-radius: 0.5rem;
        border: 1px solid var(--border-color);
        background: var(--background-color);
        color: var(--text-secondary);
        font-size: 0.75rem;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.18s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .browse-btn:hover {
        border-color: var(--primary-color);
        color: var(--primary-color);
        background: color-mix(in srgb, var(--primary-color) 8%, transparent);
    }

    .browse-btn.active {
        background: color-mix(in srgb, var(--primary-color) 12%, transparent);
        border-color: var(--primary-color);
        color: var(--primary-color);
    }

    /* Content Area */
    .ambient-content {
        display: flex;
        gap: 0.75rem;
        transition: all 0.22s cubic-bezier(0.4, 0, 0.2, 1);
    }

    /* Now Playing Inner Card */
    .now-playing-card {
        flex: 1;
        background: var(--background-color);
        border: 1px solid var(--border-color);
        border-radius: 0.75rem;
        padding: 0.875rem;
        transition: all 0.22s cubic-bezier(0.4, 0, 0.2, 1);
        min-width: 0;
    }

    .now-playing-card.compact {
        flex: 0 0 35%;
        max-width: 35%;
    }

    .now-playing-label {
        font-size: 0.75rem;
        font-weight: 500;
        color: var(--text-secondary);
        margin-bottom: 0.75rem;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        transition: opacity 0.18s ease;
    }

    .player-content {
        display: flex;
        align-items: center;
        gap: 0.75rem;
    }

    /* Track Cover with Image Support */
    .track-cover {
        width: 3.25rem;
        height: 3.25rem;
        border-radius: 0.625rem;
        background: linear-gradient(
            135deg,
            color-mix(in srgb, var(--primary-color) 100%, transparent),
            color-mix(in srgb, var(--primary-color) 70%, #8b5cf6)
        );
        display: flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;
        position: relative;
        overflow: hidden;
        transition:
            transform 0.2s cubic-bezier(0.4, 0, 0.2, 1),
            box-shadow 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .track-cover.playing {
        box-shadow:
            0 0 0 2px color-mix(in srgb, var(--primary-color) 25%, transparent),
            0 4px 12px color-mix(in srgb, var(--primary-color) 20%, transparent);
    }

    .cover-image {
        position: absolute;
        inset: 0;
        width: 100%;
        height: 100%;
        object-fit: cover;
        border-radius: 0.625rem;
    }

    .cover-overlay {
        position: absolute;
        inset: 0;
        background: linear-gradient(
            135deg,
            rgba(0, 0, 0, 0.1) 0%,
            rgba(0, 0, 0, 0.25) 100%
        );
        border-radius: 0.625rem;
        pointer-events: none;
    }

    .cover-icon {
        position: relative;
        z-index: 1;
    }

    .cover-icon svg {
        width: 1.5rem;
        height: 1.5rem;
        stroke: white;
        stroke-width: 2;
        opacity: 0.95;
    }

    /* Beat Visualizer - Atmospheric, 7 bars */
    .visualizer {
        flex: 1;
        display: flex;
        align-items: center;
        justify-content: center;
        gap: 4px;
        height: 3.25rem;
        min-width: 0;
        padding: 0 0.5rem;
        transition:
            opacity 0.22s cubic-bezier(0.4, 0, 0.2, 1),
            transform 0.22s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .visualizer.hidden {
        opacity: 0;
        transform: scale(0.95);
        pointer-events: none;
        position: absolute;
    }

    .bar {
        width: 4px;
        height: 6px;
        background: color-mix(in srgb, var(--text-secondary) 30%, transparent);
        border-radius: 2px;
        transition:
            height 0.6s cubic-bezier(0.4, 0, 0.6, 1),
            background 0.2s ease;
    }

    .visualizer.playing .bar {
        background: color-mix(in srgb, var(--primary-color) 65%, transparent);
    }

    /* Atmospheric wave animation - slow, smooth, offset delays */
    .visualizer.playing .bar:nth-child(1) {
        animation: wave 2.8s ease-in-out infinite;
        animation-delay: 0s;
    }
    .visualizer.playing .bar:nth-child(2) {
        animation: wave 3.2s ease-in-out infinite;
        animation-delay: 0.15s;
    }
    .visualizer.playing .bar:nth-child(3) {
        animation: wave 2.6s ease-in-out infinite;
        animation-delay: 0.3s;
    }
    .visualizer.playing .bar:nth-child(4) {
        animation: wave 3.4s ease-in-out infinite;
        animation-delay: 0.1s;
    }
    .visualizer.playing .bar:nth-child(5) {
        animation: wave 2.9s ease-in-out infinite;
        animation-delay: 0.25s;
    }
    .visualizer.playing .bar:nth-child(6) {
        animation: wave 3.1s ease-in-out infinite;
        animation-delay: 0.05s;
    }
    .visualizer.playing .bar:nth-child(7) {
        animation: wave 2.7s ease-in-out infinite;
        animation-delay: 0.2s;
    }

    @keyframes wave {
        0%,
        100% {
            height: 6px;
            opacity: 0.5;
        }
        50% {
            height: 22px;
            opacity: 0.85;
        }
    }

    /* Play/Pause Button */
    .play-pause-btn {
        width: 2.5rem;
        height: 2.5rem;
        border-radius: 50%;
        border: 1px solid var(--border-color);
        background: var(--surface-color);
        color: var(--text-secondary);
        cursor: pointer;
        display: flex;
        align-items: center;
        justify-content: center;
        flex-shrink: 0;
        transition: all 0.18s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .play-pause-btn:hover {
        border-color: var(--primary-color);
        color: var(--primary-color);
        background: color-mix(
            in srgb,
            var(--primary-color) 8%,
            var(--surface-color)
        );
        transform: scale(1.04);
    }

    .play-pause-btn:active {
        transform: scale(0.96);
    }

    .play-pause-btn.playing {
        background: var(--primary-color);
        border-color: var(--primary-color);
        color: white;
        box-shadow: 0 2px 8px
            color-mix(in srgb, var(--primary-color) 35%, transparent);
    }

    .play-pause-btn.playing:hover {
        background: color-mix(in srgb, var(--primary-color) 90%, black);
        color: white;
    }

    .play-pause-btn svg {
        width: 1rem;
        height: 1rem;
        transition: transform 0.15s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .play-pause-btn:active svg {
        transform: scale(0.9);
    }

    /* Track Selection Grid */
    .track-grid {
        display: none;
        grid-template-columns: repeat(3, 1fr);
        grid-template-rows: repeat(2, 1fr);
        gap: 0.5rem;
        flex: 1;
        opacity: 0;
        transform: translateX(16px);
        transition:
            opacity 0.2s cubic-bezier(0.4, 0, 0.2, 1),
            transform 0.2s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .track-grid.visible {
        display: grid;
        opacity: 1;
        transform: translateX(0);
    }

    .track-option {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 0.375rem;
        padding: 0.5rem;
        border-radius: 0.5rem;
        border: 1px solid var(--border-color);
        background: var(--background-color);
        cursor: pointer;
        transition: all 0.15s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .track-option:hover {
        border-color: color-mix(in srgb, var(--primary-color) 60%, transparent);
        background: color-mix(
            in srgb,
            var(--primary-color) 6%,
            var(--background-color)
        );
        transform: translateY(-2px);
        box-shadow: 0 2px 8px
            color-mix(in srgb, var(--primary-color) 12%, transparent);
    }

    .track-option:active {
        transform: translateY(0);
    }

    .track-option.selected {
        border-color: var(--primary-color);
        background: color-mix(
            in srgb,
            var(--primary-color) 12%,
            var(--background-color)
        );
    }

    .option-cover {
        width: 2rem;
        height: 2rem;
        border-radius: 0.375rem;
        background: linear-gradient(
            135deg,
            color-mix(in srgb, var(--primary-color) 100%, transparent),
            color-mix(in srgb, var(--primary-color) 70%, #8b5cf6)
        );
        display: flex;
        align-items: center;
        justify-content: center;
        position: relative;
        overflow: hidden;
        transition:
            transform 0.15s cubic-bezier(0.4, 0, 0.2, 1),
            box-shadow 0.15s cubic-bezier(0.4, 0, 0.2, 1);
    }

    .track-option:hover .option-cover {
        transform: scale(1.05);
    }

    .track-option.selected .option-cover {
        box-shadow: 0 0 0 2px
            color-mix(in srgb, var(--primary-color) 35%, transparent);
    }

    .option-cover-image {
        position: absolute;
        inset: 0;
        width: 100%;
        height: 100%;
        object-fit: cover;
        border-radius: 0.375rem;
    }

    .option-cover-fallback {
        width: 1rem;
        height: 1rem;
        stroke: white;
        stroke-width: 2;
        opacity: 0.9;
    }

    /* Hide fallback when image loads successfully */
    .option-cover-image + .option-cover-fallback {
        display: none;
    }

    .option-name {
        font-size: 0.625rem;
        font-weight: 500;
        color: var(--text-secondary);
        text-align: center;
        white-space: nowrap;
        overflow: hidden;
        text-overflow: ellipsis;
        max-width: 100%;
        transition: color 0.15s ease;
    }

    .track-option.selected .option-name {
        color: var(--primary-color);
        font-weight: 600;
    }

    /* Responsive adjustments */
    @media (max-width: 480px) {
        .ambient-card {
            padding: 1rem;
        }

        .now-playing-card.compact {
            flex: 0 0 30%;
            max-width: 30%;
        }

        .track-grid {
            grid-template-columns: repeat(2, 1fr);
            grid-template-rows: repeat(3, 1fr);
        }

        .visualizer {
            display: none;
        }
    }
</style>
