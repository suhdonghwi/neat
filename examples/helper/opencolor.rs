use lazy_static::lazy_static;

use ggez::graphics::Color;

macro_rules! hex {
    ($code: expr) => {
        Color::from_rgb_u32($code)
    };
}

lazy_static! {
    pub static ref WHITE: Color = hex!(0xffffff);
    pub static ref BLACK: Color = hex!(0x000000);

    // GRAY
    pub static ref GRAY0: Color = hex!(0xf8f9fa);
    pub static ref GRAY1: Color = hex!(0xf1f3f5);
    pub static ref GRAY2: Color = hex!(0xe9ecef);
    pub static ref GRAY3: Color = hex!(0xdee2e6);
    pub static ref GRAY4: Color = hex!(0xced4da);
    pub static ref GRAY5: Color = hex!(0xadb5bd);
    pub static ref GRAY6: Color = hex!(0x868e96);
    pub static ref GRAY7: Color = hex!(0x495057);
    pub static ref GRAY8: Color = hex!(0x343a40);
    pub static ref GRAY9: Color = hex!(0x212529);

    // RED
    pub static ref RED0: Color = hex!(0xfff5f5);
    pub static ref RED1: Color = hex!(0xffe3e3);
    pub static ref RED2: Color = hex!(0xffc9c9);
    pub static ref RED3: Color = hex!(0xffa8a8);
    pub static ref RED4: Color = hex!(0xff8787);
    pub static ref RED5: Color = hex!(0xff6b6b);
    pub static ref RED6: Color = hex!(0xfa5252);
    pub static ref RED7: Color = hex!(0xf03e3e);
    pub static ref RED8: Color = hex!(0xe03131);
    pub static ref RED9: Color = hex!(0xc92a2a);

    // PINK
    pub static ref PINK0: Color = hex!(0xfff0f6);
    pub static ref PINK1: Color = hex!(0xffdeeb);
    pub static ref PINK2: Color = hex!(0xfcc2d7);
    pub static ref PINK3: Color = hex!(0xfaa2c1);
    pub static ref PINK4: Color = hex!(0xf783ac);
    pub static ref PINK5: Color = hex!(0xf06595);
    pub static ref PINK6: Color = hex!(0xe64980);
    pub static ref PINK7: Color = hex!(0xd6336c);
    pub static ref PINK8: Color = hex!(0xc2255c);
    pub static ref PINK9: Color = hex!(0xa61e4d);

    // GRAPE
    pub static ref GRAPE0: Color = hex!(0xf8f0fc);
    pub static ref GRAPE1: Color = hex!(0xf3d9fa);
    pub static ref GRAPE2: Color = hex!(0xeebefa);
    pub static ref GRAPE3: Color = hex!(0xe599f7);
    pub static ref GRAPE4: Color = hex!(0xda77f2);
    pub static ref GRAPE5: Color = hex!(0xcc5de8);
    pub static ref GRAPE6: Color = hex!(0xbe4bdb);
    pub static ref GRAPE7: Color = hex!(0xae3ec9);
    pub static ref GRAPE8: Color = hex!(0x9c36b5);
    pub static ref GRAPE9: Color = hex!(0x862e9c);

    // VIOLET
    pub static ref VIOLET0: Color = hex!(0xf3f0ff);
    pub static ref VIOLET1: Color = hex!(0xe5dbff);
    pub static ref VIOLET2: Color = hex!(0xd0bfff);
    pub static ref VIOLET3: Color = hex!(0xb197fc);
    pub static ref VIOLET4: Color = hex!(0x9775fa);
    pub static ref VIOLET5: Color = hex!(0x845ef7);
    pub static ref VIOLET6: Color = hex!(0x7950f2);
    pub static ref VIOLET7: Color = hex!(0x7048e8);
    pub static ref VIOLET8: Color = hex!(0x6741d9);
    pub static ref VIOLET9: Color = hex!(0x5f3dc4);

    // INDIGO
    pub static ref INDIGO0: Color = hex!(0xedf2ff);
    pub static ref INDIGO1: Color = hex!(0xdbe4ff);
    pub static ref INDIGO2: Color = hex!(0xbac8ff);
    pub static ref INDIGO3: Color = hex!(0x91a7ff);
    pub static ref INDIGO4: Color = hex!(0x748ffc);
    pub static ref INDIGO5: Color = hex!(0x5c7cfa);
    pub static ref INDIGO6: Color = hex!(0x4c6ef5);
    pub static ref INDIGO7: Color = hex!(0x4263eb);
    pub static ref INDIGO8: Color = hex!(0x3b5bdb);
    pub static ref INDIGO9: Color = hex!(0x364fc7);

    // BLUE
    pub static ref BLUE0: Color = hex!(0xe7f5ff);
    pub static ref BLUE1: Color = hex!(0xd0ebff);
    pub static ref BLUE2: Color = hex!(0xa5d8ff);
    pub static ref BLUE3: Color = hex!(0x74c0fc);
    pub static ref BLUE4: Color = hex!(0x4dabf7);
    pub static ref BLUE5: Color = hex!(0x339af0);
    pub static ref BLUE6: Color = hex!(0x228be6);
    pub static ref BLUE7: Color = hex!(0x1c7ed6);
    pub static ref BLUE8: Color = hex!(0x1971c2);
    pub static ref BLUE9: Color = hex!(0x1864ab);

    // CYAN
    pub static ref CYAN0: Color = hex!(0xe3fafc);
    pub static ref CYAN1: Color = hex!(0xc5f6fa);
    pub static ref CYAN2: Color = hex!(0x99e9f2);
    pub static ref CYAN3: Color = hex!(0x66d9e8);
    pub static ref CYAN4: Color = hex!(0x3bc9db);
    pub static ref CYAN5: Color = hex!(0x22b8cf);
    pub static ref CYAN6: Color = hex!(0x15aabf);
    pub static ref CYAN7: Color = hex!(0x1098ad);
    pub static ref CYAN8: Color = hex!(0x0c8599);
    pub static ref CYAN9: Color = hex!(0x0b7285);

    // TEAL
    pub static ref TEAL0: Color = hex!(0xe6fcf5);
    pub static ref TEAL1: Color = hex!(0xc3fae8);
    pub static ref TEAL2: Color = hex!(0x96f2d7);
    pub static ref TEAL3: Color = hex!(0x63e6be);
    pub static ref TEAL4: Color = hex!(0x38d9a9);
    pub static ref TEAL5: Color = hex!(0x20c997);
    pub static ref TEAL6: Color = hex!(0x12b886);
    pub static ref TEAL7: Color = hex!(0x0ca678);
    pub static ref TEAL8: Color = hex!(0x099268);
    pub static ref TEAL9: Color = hex!(0x087f5b);

    // GREEN
    pub static ref GREEN0: Color = hex!(0xebfbee);
    pub static ref GREEN1: Color = hex!(0xd3f9d8);
    pub static ref GREEN2: Color = hex!(0xb2f2bb);
    pub static ref GREEN3: Color = hex!(0x8ce99a);
    pub static ref GREEN4: Color = hex!(0x69db7c);
    pub static ref GREEN5: Color = hex!(0x51cf66);
    pub static ref GREEN6: Color = hex!(0x40c057);
    pub static ref GREEN7: Color = hex!(0x37b24d);
    pub static ref GREEN8: Color = hex!(0x2f9e44);
    pub static ref GREEN9: Color = hex!(0x2b8a3e);

    // LIME
    pub static ref LIME0: Color = hex!(0xf4fce3);
    pub static ref LIME1: Color = hex!(0xe9fac8);
    pub static ref LIME2: Color = hex!(0xd8f5a2);
    pub static ref LIME3: Color = hex!(0xc0eb75);
    pub static ref LIME4: Color = hex!(0xa9e34b);
    pub static ref LIME5: Color = hex!(0x94d82d);
    pub static ref LIME6: Color = hex!(0x82c91e);
    pub static ref LIME7: Color = hex!(0x74b816);
    pub static ref LIME8: Color = hex!(0x66a80f);
    pub static ref LIME9: Color = hex!(0x5c940d);

    // YELLOW
    pub static ref YELLOW0: Color = hex!(0xfff9db);
    pub static ref YELLOW1: Color = hex!(0xfff3bf);
    pub static ref YELLOW2: Color = hex!(0xffec99);
    pub static ref YELLOW3: Color = hex!(0xffe066);
    pub static ref YELLOW4: Color = hex!(0xffd43b);
    pub static ref YELLOW5: Color = hex!(0xfcc419);
    pub static ref YELLOW6: Color = hex!(0xfab005);
    pub static ref YELLOW7: Color = hex!(0xf59f00);
    pub static ref YELLOW8: Color = hex!(0xf08c00);
    pub static ref YELLOW9: Color = hex!(0xe67700);

    // ORANGE
    pub static ref ORANGE0: Color = hex!(0xfff4e6);
    pub static ref ORANGE1: Color = hex!(0xffe8cc);
    pub static ref ORANGE2: Color = hex!(0xffd8a8);
    pub static ref ORANGE3: Color = hex!(0xffc078);
    pub static ref ORANGE4: Color = hex!(0xffa94d);
    pub static ref ORANGE5: Color = hex!(0xff922b);
    pub static ref ORANGE6: Color = hex!(0xfd7e14);
    pub static ref ORANGE7: Color = hex!(0xf76707);
    pub static ref ORANGE8: Color = hex!(0xe8590c);
    pub static ref ORANGE9: Color = hex!(0xd9480f);
}

pub fn with_alpha(mut color: Color, opacity: f32) -> Color {
    color.a = opacity;
    color
}
