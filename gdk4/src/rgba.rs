// Take a look at the license at the top of the repository in the LICENSE file.

use crate::RGBA;
use glib::translate::*;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Default)]
pub struct RGBABuilder {
    red: Option<f32>,
    green: Option<f32>,
    blue: Option<f32>,
    alpha: Option<f32>,
}

impl RGBABuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`RGBABuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    pub fn blue(mut self, blue: f32) -> Self {
        self.blue = Some(blue);
        self
    }

    pub fn green(mut self, green: f32) -> Self {
        self.green = Some(green);
        self
    }

    pub fn red(mut self, red: f32) -> Self {
        self.red = Some(red);
        self
    }

    pub fn alpha(mut self, alpha: f32) -> Self {
        self.alpha = Some(alpha);
        self
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`RGBA`].
    pub fn build(self) -> RGBA {
        let mut rgba = RGBA::WHITE;
        if let Some(blue) = self.blue {
            rgba.0.blue = blue;
        }
        if let Some(red) = self.red {
            rgba.0.red = red;
        }
        if let Some(green) = self.green {
            rgba.0.green = green;
        }
        if let Some(alpha) = self.alpha {
            rgba.0.alpha = alpha;
        }
        rgba
    }
}

impl RGBA {
    pub fn new(red: f32, green: f32, blue: f32, alpha: f32) -> RGBA {
        skip_assert_initialized!();
        RGBA(ffi::GdkRGBA {
            red,
            green,
            blue,
            alpha,
        })
    }

    pub fn red(&self) -> f32 {
        self.0.red
    }

    pub fn green(&self) -> f32 {
        self.0.green
    }

    pub fn blue(&self) -> f32 {
        self.0.blue
    }

    pub fn alpha(&self) -> f32 {
        self.0.alpha
    }

    #[doc(alias = "gdk_rgba_parse")]
    pub fn parse(s: &str) -> Result<RGBA, glib::error::BoolError> {
        skip_assert_initialized!();
        unsafe {
            let mut res = RGBA::uninitialized();
            glib::result_from_gboolean!(
                ffi::gdk_rgba_parse(res.to_glib_none_mut().0, s.to_glib_none().0),
                "Can't parse RGBA"
            )
            .map(|_| res)
        }
    }

    pub const BLACK: RGBA = RGBA(ffi::GdkRGBA {
        red: 0f32,
        green: 0f32,
        blue: 0f32,
        alpha: 1f32,
    });

    pub const BLUE: RGBA = RGBA(ffi::GdkRGBA {
        red: 0f32,
        green: 0f32,
        blue: 1f32,
        alpha: 1f32,
    });

    pub const GREEN: RGBA = RGBA(ffi::GdkRGBA {
        red: 0f32,
        green: 1f32,
        blue: 0f32,
        alpha: 1f32,
    });

    pub const RED: RGBA = RGBA(ffi::GdkRGBA {
        red: 1f32,
        green: 0f32,
        blue: 0f32,
        alpha: 1f32,
    });

    pub const WHITE: RGBA = RGBA(ffi::GdkRGBA {
        red: 1f32,
        green: 0f32,
        blue: 0f32,
        alpha: 0f32,
    });

    /* Light Sea Green rgba(27, 163, 156, 1) */
    pub const LSG: RGBA = RGBA(ffi::GdkRGBA {
        red: 0.105f32,
        green: 0.639f32,
        blue: 0.611f32,
        alpha: 1f32,
    });

    // Caribbean Green
    pub const CARIBBEAN_GREEN: RGBA = RGBA(ffi::GdkRGBA {
        red: 1f32,
        green: 1f32,
        blue: 1f32,
        alpha: 1f32,
    });

    pub const EMERALD: RGBA = RGBA(ffi::GdkRGBA {
        red: 0.247f32,
        green: 0.765f32,
        blue: 0.502f32,
        alpha: 1.000f32,
        });
        pub const LIGHT_SEA_GREEN: RGBA = RGBA(ffi::GdkRGBA {
        red: 0.106f32,
        green: 0.639f32,
        blue: 0.612f32,
        alpha: 1.000f32,
        });
        pub const NIAGARA: RGBA = RGBA(ffi::GdkRGBA {
        red: 0.165f32,
        green: 0.733f32,
        blue: 0.608f32,
        alpha: 1.000f32,
        });
        pub const JAVA: RGBA = RGBA(ffi::GdkRGBA {
        red: 0.137f32,
        green: 0.796f32,
        blue: 0.655f32,
        alpha: 1.000f32,
        });
        pub const REBECCA_PURPLE: RGBA = RGBA(ffi::GdkRGBA {
        red: 0.400f32,
        green: 0.200f32,
        blue: 0.600f32,
        alpha: 1.000f32,
        });
        pub const ELECTRIC_INDIGO: RGBA = RGBA(ffi::GdkRGBA {
        red: 0.549f32,
        green: 0.078f32,
        blue: 0.988f32,
        alpha: 1.000f32,
        });
        
    // Emerald
    //pub const WHITE: RGBA = RGBA(ffi::GdkRGBA {
    //    red: 1f32,
    //    green: 1f32,
    //    blue: 1f32,
    //    alpha: 1f32,
    //});
    // Mountain Meadow
    //pub const WHITE: RGBA = RGBA(ffi::GdkRGBA {
    //    red: 1f32,
    //    green: 1f32,
    //    blue: 1f32,
    //    alpha: 1f32,
    //});
    // Shamrock
    //pub const WHITE: RGBA = RGBA(ffi::GdkRGBA {
    //    red: 1f32,
    //    green: 1f32,
    //    blue: 1f32,
    //    alpha: 1f32,
    //});
    // Aquamarine
    //pub const WHITE: RGBA = RGBA(ffi::GdkRGBA {
    //    red: 1f32,
    //    green: 1f32,
    //    blue: 1f32,
    //    alpha: 1f32,
    //});
    // Downy
    //pub const WHITE: RGBA = RGBA(ffi::GdkRGBA {
    //    red: 1f32,
    //    green: 1f32,
    //    blue: 1f32,
    //    alpha: 1f32,
    //});
    // Jade 
    //pub const WHITE: RGBA = RGBA(ffi::GdkRGBA {
    //    red: 1f32,
    //    green: 1f32,
    //    blue: 1f32,
    //    alpha: 1f32,
    //});
    // Niagara 
    //pub const WHITE: RGBA = RGBA(ffi::GdkRGBA {
    //    red: 1f32,
    //    green: 1f32,
    //    blue: 1f32,
    //    alpha: 1f32,
    //});
    // Aqua Island
    //pub const WHITE: RGBA = RGBA(ffi::GdkRGBA {
    //    red: 1f32,
    //    green: 1f32,
    //    blue: 1f32,
    //    alpha: 1f32,
    //});
    // Observatory
    //pub const WHITE: RGBA = RGBA(ffi::GdkRGBA {
    //    red: 1f32,
    //    green: 1f32,
    //    blue: 1f32,
    //    alpha: 1f32,
    //});
    // Turquoise
    //pub const WHITE: RGBA = RGBA(ffi::GdkRGBA {
    //    red: 1f32,
    //    green: 1f32,
    //    blue: 1f32,
    //    alpha: 1f32,
    //});
}

impl fmt::Debug for RGBA {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RGBA")
            .field("red", &self.red())
            .field("green", &self.green())
            .field("blue", &self.blue())
            .field("alpha", &self.alpha())
            .finish()
    }
}

impl FromStr for RGBA {
    type Err = glib::BoolError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        skip_assert_initialized!();
        RGBA::parse(s)
    }
}
