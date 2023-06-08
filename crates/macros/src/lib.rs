use std::{
    process::Command,
};

// use chrono::{Datelike, Utc};
use proc_macro2::TokenStream;
use quote::quote;

//

#[proc_macro]
pub fn rtc_year(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if let Some(err) = expect_empty(input) {
        return err;
    }

    // hardcoding to workaround a cargo bug
    let year = 23; // Utc::now().date_naive().year();

    (quote! {
        #year
    })
    .into()
}

#[proc_macro]
pub fn build_time(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if let Some(err) = expect_empty(input) {
        return err;
    }

    // hardcoding to workaround a cargo bug
    let time = "23-06-08 21:33:33"; /* Utc::now()
                                    .naive_local()
                                    .format("%Y-%m-%d %H:%M:%S")
                                    .to_string(); */

    (quote! {
        #time
    })
    .into()
}

#[proc_macro]
pub fn build_rev(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    if let Some(err) = expect_empty(input) {
        return err;
    }

    let res = Command::new("git")
        .arg("rev-parse")
        .arg("HEAD")
        .output()
        .unwrap()
        .stdout;
    let rev = std::str::from_utf8(&res).unwrap();

    (quote! {
        #rev
    })
    .into()
}

#[proc_macro]
pub fn bmp_to_font(_input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // let path: syn::LitStr = syn::parse_macro_input!(input as syn::LitStr);
    /* let img_path = path.value();
    let path_hash = hash(&img_path);

    let cache_path = PathBuf::from(env::var("CARGO_BUILD_TARGET_DIR").unwrap())
        .join("hyperion-build-cache")
        .join(path_hash.to_string());

    fn cache_is_old(img_path: &Path, cache_path: &Path) -> io::Result<bool> {
        let img = fs::metadata(img_path)?.modified()?;
        let cache = fs::metadata(cache_path)?.modified()?;

        Ok(img > cache)
    } */

    // hardcoding to workaround a cargo bug
    let tokens: TokenStream = quote! {
        [([21845,32768,1,32768,1,35410,2647,35418,31123,32768,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,51612,19011,64076,19025,51598,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,42460,9347,39052,9361,42126,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,42462,9347,39070,9347,42142,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,64926,4675,37470,4675,37278,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,45662,19011,51934,27459,62046,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,42380,5203,35934,5203,42386,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,35790,2131,35790,2131,64462,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,40056,649,35960,4233,36472,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,40772,1093,33916,1093,33860,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,40708,261,40708,261,33148,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,40772,1093,33860,1065,33808,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,40828,261,40828,261,33028,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,36728,4357,36612,2309,37240,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,36472,4357,37176,4417,36412,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,40824,1029,33848,1089,40764,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,47260,2213,47268,2213,48028,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,37262,6227,36946,4179,47502,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,39310,8275,36946,2131,47502,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,39310,8275,38994,8275,39310,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,41358,12371,43090,14419,41358,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,51602,10839,39510,11227,51794,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,51756,22851,59532,18577,51342,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,40414,9347,40094,9347,40094,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,42188,11555,46562,9507,42284,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,37244,6917,38268,4357,37244,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,47708,19011,47692,19025,47502,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,62366,2115,35230,2563,61918,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,36472,265,34424,2057,34568,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,36464,265,34408,2121,34672,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,36408,329,34360,2089,34632,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,36424,329,34376,2121,34608,1,32768,1,32768,1,43690,],true,),([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],false),([0,0,0,0,16,16,16,16,16,16,16,0,16,16,0,0],false,),([0,0,68,68,68,68,0,0,0,0,0,0,0,0,0,0],false),([0,0,0,0,72,72,72,126,36,36,126,18,18,18,0,0],false,),([0,0,0,0,16,124,146,18,28,112,144,146,124,16,0,0,],false,),([0,0,0,0,140,82,82,44,16,16,104,148,148,98,0,0],false,),([0,0,0,0,56,68,68,40,24,148,162,66,98,156,0,0],false,),([0,0,16,16,16,16,0,0,0,0,0,0,0,0,0,0],false),([0,0,0,32,16,16,8,8,8,8,8,8,16,16,32,0],false,),([0,0,0,4,8,8,16,16,16,16,16,16,8,8,4,0],false,),([0,0,0,0,0,0,16,146,84,56,84,146,16,0,0,0],false,),([0,0,0,0,0,0,16,16,16,254,16,16,16,0,0,0],false,),([0,0,0,0,0,0,0,0,0,0,0,0,24,16,16,8],false),([0,0,0,0,0,0,0,0,0,60,0,0,0,0,0,0],false),([0,0,0,0,0,0,0,0,0,0,0,0,24,24,0,0],false),([0,0,0,0,64,64,32,16,16,8,8,4,2,2,0,0],false),([0,0,0,0,24,36,66,98,82,74,70,66,36,24,0,0],false,),([0,0,0,0,16,24,20,16,16,16,16,16,16,124,0,0],false,),([0,0,0,0,60,66,66,64,48,8,4,2,2,126,0,0],false,),([0,0,0,0,60,66,66,64,56,64,64,66,66,60,0,0],false,),([0,0,0,0,32,48,40,36,34,34,126,32,32,32,0,0],false,),([0,0,0,0,126,2,2,2,62,64,64,64,66,60,0,0],false,),([0,0,0,0,56,4,2,2,62,66,66,66,66,60,0,0],false,),([0,0,0,0,126,64,64,32,32,32,16,16,16,16,0,0],false,),([0,0,0,0,60,66,66,66,60,66,66,66,66,60,0,0],false,),([0,0,0,0,60,66,66,66,124,64,64,64,32,28,0,0],false,),([0,0,0,0,0,0,24,24,0,0,0,24,24,0,0,0],false),([0,0,0,0,0,0,24,24,0,0,0,24,16,16,8,0],false),([0,0,0,0,0,64,32,16,8,4,8,16,32,64,0,0],false,),([0,0,0,0,0,0,0,126,0,0,0,126,0,0,0,0],false),([0,0,0,0,0,2,4,8,16,32,16,8,4,2,0,0],false),([0,0,0,0,60,66,66,64,32,16,16,0,16,16,0,0],false,),([0,0,0,0,56,68,82,106,74,74,74,114,4,120,0,0],false,),([0,0,0,0,24,36,36,66,66,126,66,66,66,66,0,0],false,),([0,0,0,0,62,66,66,66,62,66,66,66,66,62,0,0],false,),([0,0,0,0,60,66,66,2,2,2,2,66,66,60,0,0],false,),([0,0,0,0,30,34,66,66,66,66,66,66,34,30,0,0],false,),([0,0,0,0,126,2,2,2,62,2,2,2,2,126,0,0],false),([0,0,0,0,126,2,2,2,62,2,2,2,2,2,0,0],false),([0,0,0,0,60,66,66,2,2,114,66,66,98,92,0,0],false,),([0,0,0,0,66,66,66,66,126,66,66,66,66,66,0,0],false,),([0,0,0,0,124,16,16,16,16,16,16,16,16,124,0,0],false,),([0,0,0,0,248,32,32,32,32,32,32,34,34,28,0,0],false,),([0,0,0,0,66,34,18,10,6,6,10,18,34,66,0,0],false,),([0,0,0,0,2,2,2,2,2,2,2,2,2,126,0,0],false),([0,0,0,0,66,66,102,102,90,90,66,66,66,66,0,0],false,),([0,0,0,0,66,70,70,74,74,82,82,98,98,66,0,0],false,),([0,0,0,0,60,66,66,66,66,66,66,66,66,60,0,0],false,),([0,0,0,0,62,66,66,66,62,2,2,2,2,2,0,0],false),([0,0,0,0,60,66,66,66,66,66,66,90,102,60,192,0],false,),([0,0,0,0,62,66,66,66,62,18,34,34,66,66,0,0],false,),([0,0,0,0,60,66,66,2,12,48,64,66,66,60,0,0],false,),([0,0,0,0,254,16,16,16,16,16,16,16,16,16,0,0],false,),([0,0,0,0,66,66,66,66,66,66,66,66,66,60,0,0],false,),([0,0,0,0,130,130,130,68,68,68,40,40,16,16,0,0],false,),([0,0,0,0,66,66,66,66,90,90,102,102,66,66,0,0],false,),([0,0,0,0,66,66,36,36,24,24,36,36,66,66,0,0],false,),([0,0,0,0,130,130,68,68,40,16,16,16,16,16,0,0],false,),([0,0,0,0,126,64,64,32,16,8,4,2,2,126,0,0],false,),([0,0,0,112,16,16,16,16,16,16,16,16,16,16,112,0],false,),([0,0,0,0,2,2,4,8,8,16,16,32,64,64,0,0],false),([0,0,0,14,8,8,8,8,8,8,8,8,8,8,14,0],false),([0,0,24,36,66,0,0,0,0,0,0,0,0,0,0,0],false),([0,0,0,0,0,0,0,0,0,0,0,0,0,0,254,0],false),([0,4,8,16,0,0,0,0,0,0,0,0,0,0,0,0],false),([0,0,0,0,0,0,60,66,64,124,66,66,98,92,0,0],false,),([0,0,0,2,2,2,58,70,66,66,66,66,70,58,0,0],false,),([0,0,0,0,0,0,60,66,2,2,2,2,66,60,0,0],false),([0,0,0,64,64,64,92,98,66,66,66,66,98,92,0,0],false,),([0,0,0,0,0,0,60,66,66,126,2,2,66,60,0,0],false,),([0,0,0,48,8,8,8,62,8,8,8,8,8,8,0,0],false),([0,0,0,0,0,64,92,34,34,34,28,4,60,66,66,60],false,),([0,0,0,2,2,2,58,70,66,66,66,66,66,66,0,0],false,),([0,0,0,16,16,0,24,16,16,16,16,16,16,124,0,0],false,),([0,0,0,32,32,0,48,32,32,32,32,32,32,32,18,12],false,),([0,0,0,2,2,2,34,18,10,6,10,18,34,66,0,0],false,),([0,0,0,24,16,16,16,16,16,16,16,16,16,124,0,0],false,),([0,0,0,0,0,0,110,146,146,146,146,146,146,146,0,0,],false,),([0,0,0,0,0,0,58,70,66,66,66,66,66,66,0,0],false,),([0,0,0,0,0,0,60,66,66,66,66,66,66,60,0,0],false,),([0,0,0,0,0,0,58,70,66,66,66,66,70,58,2,2],false,),([0,0,0,0,0,0,92,98,66,66,66,66,98,92,64,64],false,),([0,0,0,0,0,0,58,70,66,2,2,2,2,2,0,0],false),([0,0,0,0,0,0,60,66,2,12,48,64,66,60,0,0],false,),([0,0,0,0,8,8,8,62,8,8,8,8,8,48,0,0],false),([0,0,0,0,0,0,66,66,66,66,66,66,98,92,0,0],false,),([0,0,0,0,0,0,66,66,66,36,36,36,24,24,0,0],false,),([0,0,0,0,0,0,130,146,146,146,146,146,146,108,0,0,],false,),([0,0,0,0,0,0,66,66,36,24,24,36,66,66,0,0],false,),([0,0,0,0,0,0,66,66,66,66,66,100,88,64,64,60],false,),([0,0,0,0,0,0,126,64,32,16,8,4,2,126,0,0],false,),([0,0,0,48,8,8,16,16,8,4,8,16,16,8,8,48],false,),([0,0,16,16,16,16,16,16,16,16,16,16,16,16,16,16],false,),([0,0,0,12,16,16,8,8,16,32,16,8,8,16,16,12],false,),([0,0,0,140,146,98,0,0,0,0,0,0,0,0,0,0],false),([21845,32768,1,32768,1,35790,2131,35794,2131,64462,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,47502,19027,52174,19011,47682,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,47506,19027,47710,2643,35218,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,51662,19027,63950,18515,51278,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,51666,19031,63962,19027,51666,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,40238,9573,42404,9509,40238,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,35794,2135,35802,2131,64466,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,45980,18499,63884,18961,51662,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,45982,18499,63902,18947,51678,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,47570,1171,39070,8339,40082,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,48036,8485,41276,8485,39204,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,45986,2339,37154,8469,39176,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,47182,18515,51278,18499,48066,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,51278,18515,51278,18499,46018,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,33904,1169,33904,1105,33936,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,48028,16451,45452,2577,63950,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,48028,16451,45452,16913,47566,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,62350,2131,45138,16467,48014,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,41550,12883,41550,8771,61826,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,47694,16979,45646,2627,63874,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,47580,1155,39052,8337,40078,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,52124,18499,63554,18499,52124,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,37188,4461,38228,6981,37188,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,45532,19011,63948,18513,51278,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,45534,19011,63966,18499,51294,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,61852,2627,45644,16977,47502,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,55756,17443,50596,17705,55750,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,48028,4163,36940,4177,48014,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,48028,4163,37250,4611,47580,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,40816,1033,33840,1089,33848,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,62348,2131,35218,2579,61900,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,37176,6985,38200,4361,37128,1,32768,1,32768,1,43690,],true,),([21845,32768,1,32768,1,61900,2643,35294,2131,61522,1,32768,1,32768,1,43690,],true,),([0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],false),([0,0,0,0,16,16,0,16,16,16,16,16,16,16,0,0],false,),([0,0,0,0,16,16,124,146,18,18,146,124,16,16,0,0],false,),([0,0,0,0,112,8,8,8,62,8,8,8,124,134,0,0],false,),([0,0,0,0,0,66,60,36,66,66,36,60,66,0,0,0],false,),([0,0,0,0,130,68,40,16,254,16,254,16,16,16,0,0],false,),([0,0,0,0,16,16,16,16,0,0,16,16,16,16,0,0],false,),([0,0,0,0,60,66,2,60,66,66,60,64,66,60,0,0],false,),([36,36,0,0,0,0,0,0,0,0,0,0,0,0,0,0],false),([0,0,0,0,60,66,153,165,133,133,165,153,66,60,0,0,],false,),([0,0,56,64,120,68,120,0,124,0,0,0,0,0,0,0],false,),([0,0,0,0,0,72,72,36,36,18,36,36,72,72,0,0],false,),([0,0,0,0,0,0,0,0,0,0,126,64,64,64,0,0],false),([21845,32768,1,50780,10819,37836,4689,37454,1,32768,1,33728,1,32768,1,43690,],true,),([0,0,0,0,60,66,157,165,165,157,149,165,66,60,0,0,],false,),([0,0,126,0,0,0,0,0,0,0,0,0,0,0,0,0],false),([0,0,0,0,24,36,36,24,0,0,0,0,0,0,0,0],false),([0,0,0,0,16,16,16,254,16,16,16,0,254,0,0,0],false,),([0,0,0,28,34,32,24,4,2,62,0,0,0,0,0,0],false),([0,0,0,28,34,32,28,32,34,28,0,0,0,0,0,0],false,),([0,32,16,8,0,0,0,0,0,0,0,0,0,0,0,0],false),([0,0,0,0,0,0,66,66,66,66,66,66,102,154,2,1],false,),([0,0,0,0,252,94,94,94,92,80,80,80,80,80,80,0],false,),([0,0,0,0,0,0,0,0,24,24,0,0,0,0,0,0],false),([0,0,0,0,0,0,0,0,0,0,0,0,0,0,16,12],false),([0,0,0,8,12,10,8,8,8,62,0,0,0,0,0,0],false),([0,0,56,68,68,68,56,0,124,0,0,0,0,0,0,0],false,),([0,0,0,0,0,18,18,36,36,72,36,36,18,18,0,0],false,),([0,0,0,0,68,70,36,20,20,72,104,84,114,66,0,0],false,),([0,0,0,0,68,70,36,20,20,40,88,68,34,114,0,0],false,),([0,0,0,0,70,72,36,24,22,72,104,84,114,66,0,0],false,),([0,0,0,0,8,8,0,8,8,4,2,66,66,60,0,0],false),([12,48,0,0,24,36,36,66,66,126,66,66,66,66,0,0],false,),([48,12,0,0,24,36,36,66,66,126,66,66,66,66,0,0],false,),([24,36,0,0,24,36,36,66,66,126,66,66,66,66,0,0],false,),([76,50,0,0,24,36,36,66,66,126,66,66,66,66,0,0],false,),([36,36,0,0,24,36,36,66,66,126,66,66,66,66,0,0],false,),([24,36,24,0,24,36,36,66,66,126,66,66,66,66,0,0],false,),([0,0,0,0,248,20,18,18,254,18,18,18,18,242,0,0],false,),([0,0,0,0,60,66,66,2,2,2,2,66,66,60,16,12],false,),([12,48,0,0,126,2,2,2,62,2,2,2,2,126,0,0],false,),([48,12,0,0,126,2,2,2,62,2,2,2,2,126,0,0],false,),([24,36,0,0,126,2,2,2,62,2,2,2,2,126,0,0],false,),([36,36,0,0,126,2,2,2,62,2,2,2,2,126,0,0],false,),([24,96,0,0,124,16,16,16,16,16,16,16,16,124,0,0],false,),([48,12,0,0,124,16,16,16,16,16,16,16,16,124,0,0],false,),([24,36,0,0,124,16,16,16,16,16,16,16,16,124,0,0],false,),([36,36,0,0,124,16,16,16,16,16,16,16,16,124,0,0],false,),([0,0,0,0,30,34,66,66,79,66,66,66,34,30,0,0],false,),([76,50,0,0,66,70,70,74,74,82,82,98,98,66,0,0],false,),([12,48,0,0,60,66,66,66,66,66,66,66,66,60,0,0],false,),([48,12,0,0,60,66,66,66,66,66,66,66,66,60,0,0],false,),([24,36,0,0,60,66,66,66,66,66,66,66,66,60,0,0],false,),([76,50,0,0,60,66,66,66,66,66,66,66,66,60,0,0],false,),([36,36,0,0,60,66,66,66,66,66,66,66,66,60,0,0],false,),([0,0,0,0,0,0,0,66,36,24,36,66,0,0,0,0],false),([0,0,0,64,92,34,98,82,82,74,74,70,68,58,2,0],false,),([12,48,0,0,66,66,66,66,66,66,66,66,66,60,0,0],false,),([48,12,0,0,66,66,66,66,66,66,66,66,66,60,0,0],false,),([24,36,0,0,66,66,66,66,66,66,66,66,66,60,0,0],false,),([36,36,0,0,66,66,66,66,66,66,66,66,66,60,0,0],false,),([48,12,0,0,130,130,68,68,40,16,16,16,16,16,0,0],false,),([0,0,0,2,2,30,34,66,66,34,30,2,2,2,0,0],false,),([0,0,0,0,28,34,34,18,26,34,66,66,74,50,0,0],false,),([0,0,12,48,0,0,60,66,64,124,66,66,98,92,0,0],false,),([0,0,48,12,0,0,60,66,64,124,66,66,98,92,0,0],false,),([0,0,24,36,0,0,60,66,64,124,66,66,98,92,0,0],false,),([0,0,76,50,0,0,60,66,64,124,66,66,98,92,0,0],false,),([0,0,36,36,0,0,60,66,64,124,66,66,98,92,0,0],false,),([0,24,36,24,0,0,60,66,64,124,66,66,98,92,0,0],false,),([0,0,0,0,0,0,124,146,144,252,18,18,146,124,0,0],false,),([0,0,0,0,0,0,60,66,2,2,2,2,66,60,16,12],false,),([0,0,12,48,0,0,60,66,66,126,2,2,66,60,0,0],false,),([0,0,48,12,0,0,60,66,66,126,2,2,66,60,0,0],false,),([0,0,24,36,0,0,60,66,66,126,2,2,66,60,0,0],false,),([0,0,36,36,0,0,60,66,66,126,2,2,66,60,0,0],false,),([0,0,12,48,0,0,24,16,16,16,16,16,16,124,0,0],false,),([0,0,48,12,0,0,24,16,16,16,16,16,16,124,0,0],false,),([0,0,24,36,0,0,24,16,16,16,16,16,16,124,0,0],false,),([0,0,36,36,0,0,24,16,16,16,16,16,16,124,0,0],false,),([0,0,76,48,40,68,64,124,66,66,66,66,66,60,0,0],false,),([0,0,76,50,0,0,58,70,66,66,66,66,66,66,0,0],false,),([0,0,12,48,0,0,60,66,66,66,66,66,66,60,0,0],false,),([0,0,48,12,0,0,60,66,66,66,66,66,66,60,0,0],false,),([0,0,24,36,0,0,60,66,66,66,66,66,66,60,0,0],false,),([0,0,76,50,0,0,60,66,66,66,66,66,66,60,0,0],false,),([0,0,36,36,0,0,60,66,66,66,66,66,66,60,0,0],false,),([0,0,0,0,0,0,24,0,0,126,0,0,24,0,0,0],false),([0,0,0,0,0,64,60,98,82,82,74,74,70,60,2,0],false,),([0,0,12,48,0,0,66,66,66,66,66,66,98,92,0,0],false,),([0,0,48,12,0,0,66,66,66,66,66,66,98,92,0,0],false,),([0,0,24,36,0,0,66,66,66,66,66,66,98,92,0,0],false,),([0,0,36,36,0,0,66,66,66,66,66,66,98,92,0,0],false,),([0,0,48,12,0,0,66,66,66,66,66,100,88,64,64,60],false,),([0,0,0,2,2,2,58,70,66,66,66,66,70,58,2,2],false,),([0,0,36,36,0,0,66,66,66,66,66,100,88,64,64,60],false,),]
    };
    /* let tokens = if cache_is_old(img_path.as_ref(), &cache_path).unwrap_or(true) {
        let mut generated_rs = File::options()
            .write(true)
            .truncate(true)
            .open(cache_path)
            .unwrap();

        let bmp = image::open(img_path).unwrap().to_luma8();
        assert_eq!(bmp.width(), 4096);
        assert_eq!(bmp.height(), 16);

        let mut result = TokenStream::new();

        for i in 0..=255_u8 {
            let mut byte = ([0u16; 16], false);

            bmp.chunks(16)
                .skip(i as usize)
                .step_by(256)
                .enumerate()
                .for_each(|(i, s)| {
                    s.iter().enumerate().for_each(|(j, b)| {
                        if *b != 255 {
                            byte.0[i] |= 1 << j
                        }
                    })
                });

            // set the flag if the character is 16 wide instead of 8 wide
            byte.1 = !byte.0.iter().all(|c| *c < 0x100);

            let ascii_char_rows = byte
                .0
                .into_iter()
                .fold(quote! {}, |acc, s| quote! { #acc, #s });
            let is_double_wide = byte.1;

            result = quote! {
                #result
                ([#ascii_char_rows], #is_double_wide),
            };
        }

        result = quote! {
            [#result]
        };

        use std::io::Write;
        write!(generated_rs, "{result}").unwrap();
        result
    } else {
        let mut cached_rs = File::options().read(true).open(cache_path).unwrap();
        let mut buf = String::new();
        cached_rs.read_to_string(&mut buf).unwrap();

        buf.parse().unwrap()
    }; */

    tokens.into()
}

fn expect_empty(input: proc_macro::TokenStream) -> Option<proc_macro::TokenStream> {
    if !input.is_empty() {
        return Some(
            (quote! {
                compile_error!("expected zero tokens")
            })
            .into(),
        );
    }
    None
}

/* fn hash(v: impl Hash) -> u64 {
    let mut hasher = DefaultHasher::new();
    v.hash(&mut hasher);
    hasher.finish()
} */
