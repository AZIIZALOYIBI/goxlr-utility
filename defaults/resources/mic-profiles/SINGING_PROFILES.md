# Professional Singing Mic Profiles for GoXLR

This collection provides four professional-grade singing microphone profiles optimized for high-fidelity vocal performance.

## Available Profiles

### 1. SINGING.goxlrMicProfile (Enhanced General Purpose)
**Best for:** General professional singing, recordings, and versatile performance

**Key Features:**
- **EQ Curve:** Balanced vocal presence with controlled low-end and enhanced clarity
  - Low-end reduction (31Hz-125Hz): -1 to -4 dB (reduces muddiness)
  - Body enhancement (250Hz-500Hz): +2 to +3 dB (warmth and fullness)
  - Presence boost (1kHz-4kHz): +5 to +6 dB (broadcast-quality clarity)
  - Air and sparkle (8kHz-16kHz): +2 to +3 dB (professional sheen)

- **Compressor:** Transparent dynamic control
  - Threshold: -22 dB (natural dynamics preservation)
  - Ratio: 3:1 (gentle, musical compression)
  - Attack: 5ms (lets transients through for emotional expression)
  - Release: 20ms (smooth gain reduction)
  - Makeup Gain: +8 dB

- **De-esser:** 45 (professional sibilance control without losing articulation)

- **Noise Gate:** Clean, precise gating
  - Threshold: -52 dB (cleaner takes)
  - Attack: 1ms (fast, precise)
  - Release: 35ms (natural vocal tail preservation)

---

### 2. SINGING_STUDIO.goxlrMicProfile
**Best for:** Controlled studio environment, vocal recording sessions, acoustic performances

**Key Features:**
- **EQ Curve:** Studio-grade clarity with extended high-frequency detail
  - Aggressive low-end control (-2 to -5 dB at 31-125Hz)
  - Natural body (250-500Hz: +1 to +2 dB)
  - Enhanced presence (1-4kHz: +4 to +6 dB)
  - Extended air (8-16kHz: +3 to +4 dB for detail)

- **Compressor:** Gentle, transparent compression
  - Threshold: -24 dB (preserves emotional dynamics)
  - Ratio: 2.5:1 (very gentle, natural)
  - Attack: 8ms (slow, preserves natural attack)
  - Release: 30ms (smooth, musical)
  - Makeup Gain: +10 dB

- **De-esser:** 50 (strong sibilance control for pristine recordings)

- **Noise Gate:** Ultra-clean with long release
  - Threshold: -55 dB (extremely clean)
  - Attack: 1ms
  - Release: 40ms (prevents cutting vocal tails)

**Use Case:** Ideal for professional vocal tracking, acoustic sessions, and situations where you want maximum dynamic range and natural expression.

---

### 3. SINGING_LIVE.goxlrMicProfile
**Best for:** Live performances, stage singing, high-energy vocals

**Key Features:**
- **EQ Curve:** Aggressive presence for cutting through live mix
  - Strong low-end reduction (-2 to -5 dB at 31-125Hz)
  - Forward body (250-500Hz: +3 to +4 dB)
  - Strong presence boost (1-4kHz: +6 to +7 dB for projection)
  - Enhanced clarity (8-16kHz: +4 to +1 dB)

- **Compressor:** Active, controlled dynamics
  - Threshold: -20 dB (more consistent levels)
  - Ratio: 4:1 (tighter control)
  - Attack: 4ms (quick response)
  - Release: 15ms (responsive)
  - Makeup Gain: +7 dB

- **De-esser:** 40 (balanced for live clarity)

- **Noise Gate:** Responsive for stage environment
  - Threshold: -48 dB (rejects stage noise)
  - Attack: 1ms
  - Release: 30ms (quick but natural)

**Use Case:** Perfect for live stage performances, energetic vocals, and situations where you need to cut through a dense mix.

---

### 4. SINGING_BROADCAST.goxlrMicProfile
**Best for:** Streaming, podcasting, broadcasting, online content creation

**Key Features:**
- **EQ Curve:** Maximum clarity and intelligibility
  - Aggressive low-end cut (-3 to -6 dB at 31-125Hz for clean broadcast)
  - Controlled body (250-500Hz: +2 to +3 dB)
  - Strong presence (1-4kHz: +5 to +7 dB for maximum clarity)
  - Enhanced articulation (8-16kHz: +3 to +5 dB)

- **Compressor:** Broadcast-style consistency
  - Threshold: -20 dB (consistent levels for streaming)
  - Ratio: 3.5:1 (controlled dynamics)
  - Attack: 4ms (responsive)
  - Release: 18ms (fast recovery)
  - Makeup Gain: +9 dB

- **De-esser:** 55 (aggressive sibilance control for professional broadcast)

- **Noise Gate:** Precise for clean streams
  - Threshold: -50 dB (clean signal)
  - Attack: 1ms
  - Release: 32ms (balanced)

**Use Case:** Ideal for streaming, podcasting, voice-over work, and any broadcast scenario where consistent, clear vocals are critical.

---

## Installation

### Using GoXLR Utility
1. Open the GoXLR Utility application
2. Navigate to the Mic Profiles section
3. The profiles should appear automatically in your profile list
4. Double-click any profile to load it

### Manual Installation (Linux/Unix)
```bash
# Profiles are automatically installed to:
~/.config/goxlr-utility/mic-profiles/

# Or use the C installer for the main SINGING profile:
gcc bin/singing_profile.c -o singing_installer
./singing_installer
```

### Manual Installation (Windows)
Profiles should be placed in:
```
%USERPROFILE%\Documents\GoXLR\Mic Profiles\
```

---

## Technical Notes

### Understanding the Settings

**EQ (Equalizer):**
- Negative values reduce frequency content
- Positive values boost frequency content
- Professional vocals typically reduce low-end mud and boost presence/clarity

**Compressor:**
- **Threshold:** Signal level where compression begins (lower = more compression)
- **Ratio:** How much compression is applied (higher = more compression)
- **Attack:** How quickly compression responds (slower = more transient punch)
- **Release:** How quickly compression stops (slower = smoother, more musical)
- **Makeup Gain:** Compensates for volume lost during compression

**De-esser:**
- Reduces harsh "s" and "t" sounds (sibilance)
- Higher values = more de-essing
- Range: 0-100

**Gate:**
- **Threshold:** Signal level to open the gate (lower = gate opens easier)
- **Attack:** How quickly gate opens (faster = more precise)
- **Release:** How quickly gate closes (slower = preserves natural decay)

---

## Customization Tips

1. **Start with the right profile:** Choose based on your use case
2. **Adjust gate threshold:** Set it just below your normal singing level
3. **Fine-tune EQ:** Every voice and mic combination is unique
4. **Test compression:** More isn't always better - preserve dynamics when possible
5. **Monitor de-essing:** Too much can make vocals sound lispy

---

## Profile Comparison Chart

| Feature | SINGING | STUDIO | LIVE | BROADCAST |
|---------|---------|--------|------|-----------|
| Dynamic Range | Moderate | High | Controlled | Controlled |
| Presence Boost | Strong | Moderate | Very Strong | Strong |
| Low-end Control | Good | Aggressive | Moderate | Aggressive |
| De-essing | Moderate | Strong | Moderate | Very Strong |
| Compression | Balanced | Gentle | Active | Broadcast-style |
| Best For | General Use | Recording | Performance | Streaming |

---

## Support

For issues or questions about these profiles:
- Visit: https://github.com/GoXLR-on-Linux/goxlr-utility
- Discord: https://discord.gg/BRBjkkbvmZ

---

## License

These profiles are part of the GoXLR Utility project and follow the same license terms.

**Disclaimer:** These profiles are provided as-is. Settings that work for one voice/microphone combination may need adjustment for another. Always test and adjust to your specific setup.
