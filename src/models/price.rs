use serde::{Deserializer, Deserialize};


#[derive(Debug, PartialEq, Eq)]
pub struct Price {
    pub cent: u32,
    pub currency: String
}

impl<'de> Deserialize<'de> for Price {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where D: Deserializer<'de>, {
        let s: &str = Deserialize::deserialize(deserializer)?;
        let splitted = s.split(' ').collect::<Vec<&str>>();
        let price = splitted[0].split(',').collect::<Vec<&str>>();
        let first: u32 = price[0].parse().unwrap();
        let second: u32 = price[1].parse().unwrap();
        let cent: u32 = first * 100 + second;
        let currency = String::from(splitted[1]);
        Ok(Price { cent, currency })
    }
}

impl ToString for Price {
    fn to_string(&self) -> String {
        let cents = self.cent % 100;
        let euro = self.cent / 100;
        format!("{},{} {}", euro, cents, self.currency)
    }
}

impl PartialOrd for Price {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cent.cmp(&other.cent))
    }
}