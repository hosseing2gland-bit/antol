#![allow(unused)]

use super::FingerprintConfig;

/// Generate JavaScript code for fingerprint injection
/// This will be injected into the browser before any page loads
pub fn generate_injection_script(config: &FingerprintConfig) -> String {
    format!(r#"
// Anti-Detection Injection Script
// Generated at runtime - DO NOT MODIFY

(function() {{
    'use strict';
    
    const CONFIG = {config_json};
    
    // ============= Navigator Overrides =============
    
    Object.defineProperty(navigator, 'platform', {{
        get: () => CONFIG.platform
    }});
    
    Object.defineProperty(navigator, 'userAgent', {{
        get: () => CONFIG.user_agent
    }});
    
    Object.defineProperty(navigator, 'vendor', {{
        get: () => CONFIG.vendor
    }});
    
    Object.defineProperty(navigator, 'hardwareConcurrency', {{
        get: () => CONFIG.hardware_concurrency
    }});
    
    Object.defineProperty(navigator, 'deviceMemory', {{
        get: () => CONFIG.device_memory
    }});
    
    Object.defineProperty(navigator, 'maxTouchPoints', {{
        get: () => CONFIG.max_touch_points
    }});
    
    Object.defineProperty(navigator, 'language', {{
        get: () => CONFIG.language
    }});
    
    Object.defineProperty(navigator, 'languages', {{
        get: () => CONFIG.languages
    }});
    
    Object.defineProperty(navigator, 'doNotTrack', {{
        get: () => CONFIG.do_not_track
    }});
    
    // ============= Screen Overrides =============
    
    Object.defineProperties(screen, {{
        width: {{ get: () => CONFIG.screen_width }},
        height: {{ get: () => CONFIG.screen_height }},
        availWidth: {{ get: () => CONFIG.available_width }},
        availHeight: {{ get: () => CONFIG.available_height }},
        colorDepth: {{ get: () => CONFIG.color_depth }},
        pixelDepth: {{ get: () => CONFIG.pixel_depth }}
    }});
    
    // ============= Timezone Overrides =============
    
    const OriginalDate = Date;
    Date = new Proxy(OriginalDate, {{
        construct(target, args) {{
            const date = new target(...args);
            const originalGetTimezoneOffset = date.getTimezoneOffset;
            date.getTimezoneOffset = function() {{
                return CONFIG.timezone_offset;
            }};
            return date;
        }},
        apply(target, thisArg, args) {{
            return new target(...args);
        }}
    }});
    Date.prototype = OriginalDate.prototype;
    
    // ============= Canvas Fingerprint Noise =============
    
    if (CONFIG.canvas_noise) {{
        const originalToDataURL = HTMLCanvasElement.prototype.toDataURL;
        const originalToBlob = HTMLCanvasElement.prototype.toBlob;
        const originalGetImageData = CanvasRenderingContext2D.prototype.getImageData;
        
        function addCanvasNoise(canvas) {{
            const ctx = canvas.getContext('2d');
            if (!ctx) return;
            
            const imageData = ctx.getImageData(0, 0, canvas.width, canvas.height);
            const data = imageData.data;
            
            // Seeded random based on fingerprint_seed
            let seed = CONFIG.fingerprint_seed.split('').reduce((a, b) => {{
                a = ((a << 5) - a) + b.charCodeAt(0);
                return a & a;
            }}, 0);
            
            const random = () => {{
                seed = (seed * 9301 + 49297) % 233280;
                return seed / 233280;
            }};
            
            // Add subtle noise to pixels
            for (let i = 0; i < data.length; i += 4) {{
                const noise = (random() - 0.5) * CONFIG.canvas_noise_level * 255;
                data[i] = Math.min(255, Math.max(0, data[i] + noise));     // R
                data[i+1] = Math.min(255, Math.max(0, data[i+1] + noise)); // G
                data[i+2] = Math.min(255, Math.max(0, data[i+2] + noise)); // B
            }}
            
            ctx.putImageData(imageData, 0, 0);
        }}
        
        HTMLCanvasElement.prototype.toDataURL = function() {{
            addCanvasNoise(this);
            return originalToDataURL.apply(this, arguments);
        }};
        
        HTMLCanvasElement.prototype.toBlob = function() {{
            addCanvasNoise(this);
            return originalToBlob.apply(this, arguments);
        }};
        
        CanvasRenderingContext2D.prototype.getImageData = function() {{
            const imageData = originalGetImageData.apply(this, arguments);
            // Add noise to ImageData
            return imageData;
        }};
    }}
    
    // ============= WebGL Fingerprint Spoofing =============
    
    if (CONFIG.webgl_noise) {{
        const getParameter = WebGLRenderingContext.prototype.getParameter;
        
        WebGLRenderingContext.prototype.getParameter = function(parameter) {{
            if (parameter === 37445) {{ // UNMASKED_VENDOR_WEBGL
                return CONFIG.webgl_vendor;
            }}
            if (parameter === 37446) {{ // UNMASKED_RENDERER_WEBGL
                return CONFIG.webgl_renderer;
            }}
            return getParameter.apply(this, arguments);
        }};
        
        // Also for WebGL2
        if (typeof WebGL2RenderingContext !== 'undefined') {{
            WebGL2RenderingContext.prototype.getParameter = WebGLRenderingContext.prototype.getParameter;
        }}
    }}
    
    // ============= Audio Context Fingerprint Noise =============
    
    if (CONFIG.audio_noise) {{
        const AudioContext = window.AudioContext || window.webkitAudioContext;
        if (AudioContext) {{
            const originalCreateAnalyser = AudioContext.prototype.createAnalyser;
            AudioContext.prototype.createAnalyser = function() {{
                const analyser = originalCreateAnalyser.apply(this, arguments);
                const originalGetFloatFrequencyData = analyser.getFloatFrequencyData;
                
                analyser.getFloatFrequencyData = function(array) {{
                    originalGetFloatFrequencyData.apply(this, arguments);
                    // Add subtle noise
                    for (let i = 0; i < array.length; i++) {{
                        array[i] += (Math.random() - 0.5) * CONFIG.audio_noise_level;
                    }}
                }};
                
                return analyser;
            }};
        }}
    }}
    
    // ============= Media Devices Spoofing =============
    
    if (CONFIG.fake_media_devices && navigator.mediaDevices) {{
        const originalEnumerateDevices = navigator.mediaDevices.enumerateDevices;
        
        navigator.mediaDevices.enumerateDevices = async function() {{
            return CONFIG.media_devices;
        }};
    }}
    
    // ============= WebRTC Leak Protection =============
    
    if (CONFIG.webrtc_leak_protection) {{
        // Disable WebRTC to prevent IP leaks
        if (window.RTCPeerConnection) {{
            window.RTCPeerConnection = function() {{
                throw new Error('WebRTC is disabled for privacy');
            }};
        }}
        if (window.webkitRTCPeerConnection) {{
            window.webkitRTCPeerConnection = function() {{
                throw new Error('WebRTC is disabled for privacy');
            }};
        }}
        if (window.mozRTCPeerConnection) {{
            window.mozRTCPeerConnection = function() {{
                throw new Error('WebRTC is disabled for privacy');
            }};
        }}
    }}
    
    // ============= Client Rects Noise =============
    
    if (CONFIG.client_rects_noise) {{
        const originalGetBoundingClientRect = Element.prototype.getBoundingClientRect;
        
        Element.prototype.getBoundingClientRect = function() {{
            const rect = originalGetBoundingClientRect.apply(this, arguments);
            const noise = 0.0001;
            
            return {{
                x: rect.x + (Math.random() - 0.5) * noise,
                y: rect.y + (Math.random() - 0.5) * noise,
                width: rect.width + (Math.random() - 0.5) * noise,
                height: rect.height + (Math.random() - 0.5) * noise,
                top: rect.top + (Math.random() - 0.5) * noise,
                right: rect.right + (Math.random() - 0.5) * noise,
                bottom: rect.bottom + (Math.random() - 0.5) * noise,
                left: rect.left + (Math.random() - 0.5) * noise,
                toJSON: rect.toJSON
            }};
        }};
    }}
    
    // ============= Battery API Spoofing =============
    
    if (CONFIG.battery_spoofing && navigator.getBattery) {{
        const originalGetBattery = navigator.getBattery;
        
        navigator.getBattery = async function() {{
            const battery = await originalGetBattery.apply(this, arguments);
            
            Object.defineProperties(battery, {{
                level: {{ get: () => CONFIG.battery_level || 1.0 }},
                charging: {{ get: () => CONFIG.battery_charging || true }},
                chargingTime: {{ get: () => 0 }},
                dischargingTime: {{ get: () => Infinity }}
            }});
            
            return battery;
        }};
    }}
    
    // ============= Font Detection Protection =============
    
    // Make only configured fonts available for detection
    const originalGetComputedStyle = window.getComputedStyle;
    window.getComputedStyle = function() {{
        const style = originalGetComputedStyle.apply(this, arguments);
        const originalGetPropertyValue = style.getPropertyValue;
        
        style.getPropertyValue = function(property) {{
            if (property === 'font-family') {{
                // Return random font from allowed list
                const randomFont = CONFIG.fonts[Math.floor(Math.random() * CONFIG.fonts.length)];
                return randomFont;
            }}
            return originalGetPropertyValue.apply(this, arguments);
        }};
        
        return style;
    }};
    
    // ============= Console Protection =============
    
    console.log('%c🛡️ Anti-Detection Active', 'color: #00ff00; font-weight: bold; font-size: 14px;');
    console.log('%cFingerprint ID:', 'color: #888;', CONFIG.fingerprint_seed);
    
    // Prevent detection of our injection
    delete window.CONFIG;
    
}})();
"#, 
        config_json = serde_json::to_string_pretty(config).unwrap_or_default()
    )
}

/// Generate content script for Chrome extension (alternative injection method)
pub fn generate_content_script(config: &FingerprintConfig) -> String {
    format!(r#"
// Content Script for Anti-Detection
// Runs before page scripts

const script = document.createElement('script');
script.textContent = `{}`;
document.documentElement.appendChild(script);
script.remove();
"#, 
        generate_injection_script(config)
            .replace('\\', "\\\\")
            .replace('`', "\\`")
            .replace('$', "\\$")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_generate_injection_script() {
        let config = FingerprintConfig::default();
        let script = generate_injection_script(&config);
        
        assert!(script.contains("Anti-Detection Injection Script"));
        assert!(script.contains(&config.user_agent));
        assert!(script.contains(&config.platform));
    }
}
