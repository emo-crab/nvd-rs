pub(crate) fn lookup(
  eq1: &u32,
  eq2: &u32,
  eq3: &u32,
  eq4: &u32,
  eq5: &u32,
  eq6: &u32,
) -> Option<f32> {
  let mv = format!("{}{}{}{}{}{}", eq1, eq2, eq3, eq4, eq5, eq6);
  match mv.as_str() {
    "000000" => Some(10.0),
    "000001" => Some(9.9),
    "000010" => Some(9.8),
    "000011" => Some(9.5),
    "000020" => Some(9.5),
    "000021" => Some(9.2),
    "000100" => Some(10.0),
    "000101" => Some(9.6),
    "000110" => Some(9.3),
    "000111" => Some(8.7),
    "000120" => Some(9.1),
    "000121" => Some(8.1),
    "000200" => Some(9.3),
    "000201" => Some(9.0),
    "000210" => Some(8.9),
    "000211" => Some(8.0),
    "000220" => Some(8.1),
    "000221" => Some(6.8),
    "001000" => Some(9.8),
    "001001" => Some(9.5),
    "001010" => Some(9.5),
    "001011" => Some(9.2),
    "001020" => Some(9.0),
    "001021" => Some(8.4),
    "001100" => Some(9.3),
    "001101" => Some(9.2),
    "001110" => Some(8.9),
    "001111" => Some(8.1),
    "001120" => Some(8.1),
    "001121" => Some(6.5),
    "001200" => Some(8.8),
    "001201" => Some(8.0),
    "001210" => Some(7.8),
    "001211" => Some(7.0),
    "001220" => Some(6.9),
    "001221" => Some(4.8),
    "002001" => Some(9.2),
    "002011" => Some(8.2),
    "002021" => Some(7.2),
    "002101" => Some(7.9),
    "002111" => Some(6.9),
    "002121" => Some(5.0),
    "002201" => Some(6.9),
    "002211" => Some(5.5),
    "002221" => Some(2.7),
    "010000" => Some(9.9),
    "010001" => Some(9.7),
    "010010" => Some(9.5),
    "010011" => Some(9.2),
    "010020" => Some(9.2),
    "010021" => Some(8.5),
    "010100" => Some(9.5),
    "010101" => Some(9.1),
    "010110" => Some(9.0),
    "010111" => Some(8.3),
    "010120" => Some(8.4),
    "010121" => Some(7.1),
    "010200" => Some(9.2),
    "010201" => Some(8.1),
    "010210" => Some(8.2),
    "010211" => Some(7.1),
    "010220" => Some(7.2),
    "010221" => Some(5.3),
    "011000" => Some(9.5),
    "011001" => Some(9.3),
    "011010" => Some(9.2),
    "011011" => Some(8.5),
    "011020" => Some(8.5),
    "011021" => Some(7.3),
    "011100" => Some(9.2),
    "011101" => Some(8.2),
    "011110" => Some(8.0),
    "011111" => Some(7.2),
    "011120" => Some(7.0),
    "011121" => Some(5.9),
    "011200" => Some(8.4),
    "011201" => Some(7.0),
    "011210" => Some(7.1),
    "011211" => Some(5.2),
    "011220" => Some(5.0),
    "011221" => Some(3.0),
    "012001" => Some(8.6),
    "012011" => Some(7.5),
    "012021" => Some(5.2),
    "012101" => Some(7.1),
    "012111" => Some(5.2),
    "012121" => Some(2.9),
    "012201" => Some(6.3),
    "012211" => Some(2.9),
    "012221" => Some(1.7),
    "100000" => Some(9.8),
    "100001" => Some(9.5),
    "100010" => Some(9.4),
    "100011" => Some(8.7),
    "100020" => Some(9.1),
    "100021" => Some(8.1),
    "100100" => Some(9.4),
    "100101" => Some(8.9),
    "100110" => Some(8.6),
    "100111" => Some(7.4),
    "100120" => Some(7.7),
    "100121" => Some(6.4),
    "100200" => Some(8.7),
    "100201" => Some(7.5),
    "100210" => Some(7.4),
    "100211" => Some(6.3),
    "100220" => Some(6.3),
    "100221" => Some(4.9),
    "101000" => Some(9.4),
    "101001" => Some(8.9),
    "101010" => Some(8.8),
    "101011" => Some(7.7),
    "101020" => Some(7.6),
    "101021" => Some(6.7),
    "101100" => Some(8.6),
    "101101" => Some(7.6),
    "101110" => Some(7.4),
    "101111" => Some(5.8),
    "101120" => Some(5.9),
    "101121" => Some(5.0),
    "101200" => Some(7.2),
    "101201" => Some(5.7),
    "101210" => Some(5.7),
    "101211" => Some(5.2),
    "101220" => Some(5.2),
    "101221" => Some(2.5),
    "102001" => Some(8.3),
    "102011" => Some(7.0),
    "102021" => Some(5.4),
    "102101" => Some(6.5),
    "102111" => Some(5.8),
    "102121" => Some(2.6),
    "102201" => Some(5.3),
    "102211" => Some(2.1),
    "102221" => Some(1.3),
    "110000" => Some(9.5),
    "110001" => Some(9.0),
    "110010" => Some(8.8),
    "110011" => Some(7.6),
    "110020" => Some(7.6),
    "110021" => Some(7.0),
    "110100" => Some(9.0),
    "110101" => Some(7.7),
    "110110" => Some(7.5),
    "110111" => Some(6.2),
    "110120" => Some(6.1),
    "110121" => Some(5.3),
    "110200" => Some(7.7),
    "110201" => Some(6.6),
    "110210" => Some(6.8),
    "110211" => Some(5.9),
    "110220" => Some(5.2),
    "110221" => Some(3.0),
    "111000" => Some(8.9),
    "111001" => Some(7.8),
    "111010" => Some(7.6),
    "111011" => Some(6.7),
    "111020" => Some(6.2),
    "111021" => Some(5.8),
    "111100" => Some(7.4),
    "111101" => Some(5.9),
    "111110" => Some(5.7),
    "111111" => Some(5.7),
    "111120" => Some(4.7),
    "111121" => Some(2.3),
    "111200" => Some(6.1),
    "111201" => Some(5.2),
    "111210" => Some(5.7),
    "111211" => Some(2.9),
    "111220" => Some(2.4),
    "111221" => Some(1.6),
    "112001" => Some(7.1),
    "112011" => Some(5.9),
    "112021" => Some(3.0),
    "112101" => Some(5.8),
    "112111" => Some(2.6),
    "112121" => Some(1.5),
    "112201" => Some(2.3),
    "112211" => Some(1.3),
    "112221" => Some(0.6),
    "200000" => Some(9.3),
    "200001" => Some(8.7),
    "200010" => Some(8.6),
    "200011" => Some(7.2),
    "200020" => Some(7.5),
    "200021" => Some(5.8),
    "200100" => Some(8.6),
    "200101" => Some(7.4),
    "200110" => Some(7.4),
    "200111" => Some(6.1),
    "200120" => Some(5.6),
    "200121" => Some(3.4),
    "200200" => Some(7.0),
    "200201" => Some(5.4),
    "200210" => Some(5.2),
    "200211" => Some(4.0),
    "200220" => Some(4.0),
    "200221" => Some(2.2),
    "201000" => Some(8.5),
    "201001" => Some(7.5),
    "201010" => Some(7.4),
    "201011" => Some(5.5),
    "201020" => Some(6.2),
    "201021" => Some(5.1),
    "201100" => Some(7.2),
    "201101" => Some(5.7),
    "201110" => Some(5.5),
    "201111" => Some(4.1),
    "201120" => Some(4.6),
    "201121" => Some(1.9),
    "201200" => Some(5.3),
    "201201" => Some(3.6),
    "201210" => Some(3.4),
    "201211" => Some(1.9),
    "201220" => Some(1.9),
    "201221" => Some(0.8),
    "202001" => Some(6.4),
    "202011" => Some(5.1),
    "202021" => Some(2.0),
    "202101" => Some(4.7),
    "202111" => Some(2.1),
    "202121" => Some(1.1),
    "202201" => Some(2.4),
    "202211" => Some(0.9),
    "202221" => Some(0.4),
    "210000" => Some(8.8),
    "210001" => Some(7.5),
    "210010" => Some(7.3),
    "210011" => Some(5.3),
    "210020" => Some(6.0),
    "210021" => Some(5.0),
    "210100" => Some(7.3),
    "210101" => Some(5.5),
    "210110" => Some(5.9),
    "210111" => Some(4.0),
    "210120" => Some(4.1),
    "210121" => Some(2.0),
    "210200" => Some(5.4),
    "210201" => Some(4.3),
    "210210" => Some(4.5),
    "210211" => Some(2.2),
    "210220" => Some(2.0),
    "210221" => Some(1.1),
    "211000" => Some(7.5),
    "211001" => Some(5.5),
    "211010" => Some(5.8),
    "211011" => Some(4.5),
    "211020" => Some(4.0),
    "211021" => Some(2.1),
    "211100" => Some(6.1),
    "211101" => Some(5.1),
    "211110" => Some(4.8),
    "211111" => Some(1.8),
    "211120" => Some(2.0),
    "211121" => Some(0.9),
    "211200" => Some(4.6),
    "211201" => Some(1.8),
    "211210" => Some(1.7),
    "211211" => Some(0.7),
    "211220" => Some(0.8),
    "211221" => Some(0.2),
    "212001" => Some(5.3),
    "212011" => Some(2.4),
    "212021" => Some(1.4),
    "212101" => Some(2.4),
    "212111" => Some(1.2),
    "212121" => Some(0.5),
    "212201" => Some(1.0),
    "212211" => Some(0.3),
    "212221" => Some(0.1),
    _ => None,
  }
}

pub(crate) fn get_eq1245_max_composed(eq: u32, index: u32) -> Vec<&'static str> {
  match eq {
    1 => match index {
      0 => vec!["AV:N/PR:N/UI:N/"],
      1 => vec!["AV:A/PR:N/UI:N/", "AV:N/PR:L/UI:N/", "AV:N/PR:N/UI:P/"],
      2 => vec!["AV:P/PR:N/UI:N/", "AV:A/PR:L/UI:P/"],
      _ => vec![],
    },
    2 => match index {
      0 => vec!["AC:L/AT:N/"],
      1 => vec!["AC:H/AT:N/", "AC:L/AT:P/"],
      _ => vec![],
    },
    4 => match index {
      0 => vec!["SC:H/SI:S/SA:S/"],
      1 => vec!["SC:H/SI:H/SA:H/"],
      2 => vec!["SC:L/SI:L/SA:L/"],
      _ => vec![],
    },
    5 => match index {
      0 => vec!["E:A/"],
      1 => vec!["E:P/"],
      2 => vec!["E:U/"],
      _ => vec![],
    },
    _ => vec![],
  }
}
pub(crate) fn get_eq36_max_composed(eq3: u32, eq6: u32) -> Vec<&'static str> {
  match eq3 {
    0 => match eq6 {
      0 => vec!["VC:H/VI:H/VA:H/CR:H/IR:H/AR:H/"],
      1 => vec![
        "VC:H/VI:H/VA:L/CR:M/IR:M/AR:H/",
        "VC:H/VI:H/VA:H/CR:M/IR:M/AR:M/",
      ],
      _ => vec![],
    },
    1 => match eq6 {
      0 => vec![
        "VC:L/VI:H/VA:H/CR:H/IR:H/AR:H/",
        "VC:H/VI:L/VA:H/CR:H/IR:H/AR:H/",
      ],
      1 => vec![
        "VC:L/VI:H/VA:L/CR:H/IR:M/AR:H/",
        "VC:L/VI:H/VA:H/CR:H/IR:M/AR:M/",
        "VC:H/VI:L/VA:H/CR:M/IR:H/AR:M/",
        "VC:H/VI:L/VA:L/CR:M/IR:H/AR:H/",
        "VC:L/VI:L/VA:H/CR:H/IR:H/AR:M/",
      ],
      _ => vec![],
    },
    2 => match eq6 {
      1 => vec!["VC:L/VI:L/VA:L/CR:H/IR:H/AR:H/"],
      _ => vec![],
    },
    _ => vec![],
  }
}

pub(crate) fn get_eq1245_max_severity(eq: u32, index: u32) -> Option<f32> {
  match eq {
    1 => match index {
      0 => Some(1.0),
      1 => Some(4.0),
      2 => Some(5.0),
      _ => None,
    },
    2 => match index {
      0 => Some(1.0),
      1 => Some(2.0),
      _ => None,
    },
    4 => match index {
      0 => Some(6.0),
      1 => Some(5.0),
      2 => Some(4.0),
      _ => None,
    },
    5 => match index {
      0 => Some(1.0),
      1 => Some(1.0),
      2 => Some(1.0),
      _ => None,
    },
    _ => None,
  }
}
pub(crate) fn get_eq36_max_severity(eq3: u32, eq6: u32) -> Option<f32> {
  match eq3 {
    0 => match eq6 {
      0 => Some(7.0),
      1 => Some(6.0),
      _ => None,
    },
    1 => match eq6 {
      0 => Some(8.0),
      1 => Some(8.0),
      _ => None,
    },
    2 => match eq6 {
      1 => Some(10.0),
      _ => None,
    },
    _ => None,
  }
}
