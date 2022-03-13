pub mod avatar;
pub mod userinfo;
mod eval;


use avatar::*;
use userinfo::*;
use eval::*;

use serenity::framework::standard::macros::group;

#[group]
#[commands(avatar, userinfo,eval)]
struct General;
