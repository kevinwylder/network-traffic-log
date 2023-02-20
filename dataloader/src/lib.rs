use memmap::Mmap;
use ndarray::{ArrayBase, Dim, ViewRepr};
use std::fs::File;
use std::io;

#[cfg(feature = "py")]
mod py;

#[cfg(feature = "go")]
mod go;

type Float = f32;

/**
 * 1st byte
 * byte    0-255 11111111
 * weekday 0-6   11100000
 * hour    0-23  00011111
 *
 * 2nd byte
 * byte    0-255 11111111
 * minute  0-60  11111100
 * unused  0     00000011
 * 
 * 3rd byte, various tags
 * 
 * 4th byte, log_1.15(traffic bytes)
 */


// EXP_TABLE is a lookup table to compute 1.15**X where X is an integer in (0, 255)
#[rustfmt::skip] 
const EXP_TABLE: [Float; 256] = [
    1.0,                1.15,               1.3224999999999998, 1.5208749999999998, 1.7490062499999994,
    2.0113571874999994, 2.313060765624999,  2.6600198804687487, 3.0590228625390607, 3.5178762919199196, 4.045557735707907,
    4.652391396064092,  5.350250105473706,  6.152787621294761,  7.075705764488975,  8.137061629162321,  9.357620873536668,
    10.761264004567169, 12.375453605252241, 14.231771646040077, 16.36653739294609,  18.821518001888,    21.644745702171196,
    24.891457557496874, 28.625176191121405, 32.918952619789614, 37.856795512758055, 43.535314839671756, 50.06561206562252,
    57.57545387546589,  66.21177195678577,  76.14353775030362,  87.56506841284916,  100.69982867477653, 115.804802975993,
    133.17552342239193, 153.15185193575073, 176.1246297261133,  202.5433241850303,  232.9248228127848,  267.86354623470254,
    308.04307816990786, 354.24953989539404, 407.3869708797031,  468.49501651165855, 538.7692689884072,  619.5846593366683,
    712.5223582371685,  819.4007119727437,  942.3108187686552,  1083.6574415839534, 1246.2060578215462, 1433.136966494778,
    1648.1075114689947, 1895.323638189344,  2179.622183917745,  2506.565511505407,  2882.5503382312177, 3314.9328889659,
    3812.172822310785,  4383.998745657402,  5041.598557506012,  5797.838341131914,  6667.5140923017,    7667.641206146955,
    8817.787387068996,  10140.455495129345, 11661.523819398746, 13410.752392308557, 15422.365251154839, 17735.720038828065,
    20396.078044652273, 23455.489751350113, 26973.813214052625, 31019.885196160518, 35672.867975584595, 41023.79817192228,
    47177.36789771062,  54253.97308236721,  62392.06904472228,  71750.87940143062,  82513.5113116452,   94890.53800839197,
    109124.11870965076, 125492.73651609836, 144316.6469935131,  165964.14404254008, 190858.76564892105, 219487.5804962592,
    252410.71757069806, 290272.32520630275, 333813.17398724816, 383885.1500853353,  441467.9225981356,  507688.1109878559,
    583841.3276360342,  671417.5267814393,  772130.1557986551,  887949.6791684533,  1021142.1310437213, 1174313.4507002793,
    1350460.4683053212, 1553029.5385511192, 1785983.969333787,  2053881.564733855,  2361963.7994439327, 2716258.369360523,
    3123697.124764601,  3592251.6934792907, 4131089.447501184,  4750752.864626361,  5463365.794320315,  6282870.663468362,
    7225301.262988616,  8309096.452436907,  9555460.920302443,  10988780.058347808, 12637097.067099977, 14532661.627164973,
    16712560.871239718, 19219445.001925673, 22102361.752214525, 25417716.0150467,   29230373.417303704, 33614929.42989926,
    38657168.84438414,  44455744.17104176,  51124105.79669802,  58792721.66620272,  67611629.91613312,  77753374.40355308,
    89416380.56408603,  102828837.64869894, 118253163.29600377, 135991137.79040432, 156389808.45896497, 179848279.7278097,
    206825521.68698114, 237849349.94002828, 273526752.4310325,  314555765.2956874,  361739130.09004045, 415999999.60354644,
    478399999.5440784,  550159999.4756901,  632683999.3970436,  727586599.3066001,  836724589.20259,    962233277.5829784,
    1106568269.2204251, 1272553509.6034887, 1463436536.044012,  1682952016.4506137, 1935394818.9182055, 2225704041.755936,
    2559559648.019326,  2943493595.222225,  3385017634.5055585, 3892770279.681392,  4476685821.6336,    5148188694.87864,
    5920416999.1104355, 6808479548.977001,  7829751481.32355,   9004214203.522081,  10354846334.050394, 11908073284.157951,
    13694284276.781643, 15748426918.29889,  18110690956.04372,  20827294599.450275, 23951388789.367817, 27544097107.772987,
    31675711673.938934, 36427068425.02977,  41891128688.78423,  48174797992.10186,  55401017690.91714,  63711170344.5547,
    73267845896.2379,   84258022780.67358,  96896726197.77461,  111431235127.4408,  128145920396.5569,  147367808456.04044,
    169472979724.44647, 194893926683.11343, 224128015685.58044, 257747218038.41748, 296409300744.1801,  340870695855.80707,
    392001300234.1781,  450801495269.3048,  518421719559.70044, 596184977493.6555,  685612724117.7037,  788454632735.3593,
    906722827645.6631,  1042731251792.5125, 1199140939561.3892, 1379012080495.5974, 1585863892569.937,  1823743476455.4275,
    2097304997923.7415, 2411900747612.3022, 2773685859754.1475, 3189738738717.2695, 3668199549524.8594, 4218429481953.5884,
    4851193904246.626,  5578872989883.619,  6415703938366.162,  7378059529121.086,  8484768458489.248,  9757483727262.635,
    11221106286352.03,  12904272229304.832, 14839913063700.555, 17065900023255.637, 19625785026743.98,  22569652780755.58,
    25955100697868.91,  29848365802549.246, 34325620672931.63,  39474463773871.375, 45395633339952.07,  52204978340944.88,
    60035725092086.61,  69041083855899.59,  79397246434284.53,  91306833399427.2,   105002858409341.27, 120753287170742.45,
    138866280246353.81, 159696222283306.88, 183650655625802.88, 211198253969673.3,  242877992065124.28, 279309690874892.9,
    321206144506126.8,  369387066182045.8,  424795126109352.6,  488514395025755.5,  561791554279618.75, 646060287421561.5,
    742969330534795.8,  854414730115015.0,  982576939632267.1,  1129963480577107.2, 1299458002663673.2, 1494376703063224.0,
    1718533208522707.5, 1976313189801113.5, 2272760168271280.5, 2613674193511972.0, 3005725322538767.5,
];

type MutableWeek<'a> = ArrayBase<ViewRepr<&'a mut Float>, Dim<[usize; 2]>>;

fn open_data_file(path: &str) -> io::Result<Mmap> {
    let f = File::open(path)?;
    unsafe { Mmap::map(&f) }
}

fn graph_traffic(mut array: MutableWeek, samples: usize, path: &str) -> io::Result<()> {
    let data = open_data_file(path)?;
    for i in (0..data.len()).step_by(4) {
        let day = (data[i] >> 5) as usize;
        let minute = ((data[i] & 0x1F) as usize * 60) + (data[i + 1] >> 2) as usize;
        let sample = minute * samples / (24 * 60);
        array[(day, sample)] += EXP_TABLE[data[i + 3] as usize];
    }
    Ok(())
}
