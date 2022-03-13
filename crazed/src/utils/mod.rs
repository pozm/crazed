pub(crate) mod http;

pub mod embed {
    use chrono::Utc;
    use serenity::builder::CreateEmbed;
    use std::time::SystemTime;

    pub fn default(e: &mut CreateEmbed) -> &mut CreateEmbed {
        e.colour(0xD779BC)
            .author(|a|{
                a.name("crazed")
                    .icon_url("https://cdn.discordapp.com/attachments/952158497460600945/952463558581571624/Salmon.gif") // fishe
            })
            .footer(|f| {
                f.text("❤")
            })
            .timestamp(chrono::DateTime::<Utc>::from(SystemTime::now()))
    }
}
pub mod macros {
    #[macro_export()]
    macro_rules! use_commands {
        (($e:expr),*) => {
            $(
                pub mod $e;
                use $e::*
            )*
            #[group]
            #[commands($($e,)*)]
        };
    }
}

pub mod configuration {
    pub struct Guild;
    impl Guild {
        #[inline]
        pub fn default_guild() -> u64 {
            902738459368755281u64
        }
    }
}
