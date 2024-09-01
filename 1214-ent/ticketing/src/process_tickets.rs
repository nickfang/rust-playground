use std::collections::BTreeMap;
use std::error::Error;
use std::fs::File;
use std::io::{ BufReader, BufWriter };

use csv::{ ReaderBuilder, WriterBuilder };
use glob::glob;
use serde::{ Deserialize, Deserializer };

fn deserialize_f32_or_zero<'de, D>(deserializer: D) -> Result<f32, D::Error>
  where D: Deserializer<'de>
{
  let s: String = Deserialize::deserialize(deserializer)?;
  match s.parse() {
    Ok(f) => Ok(f),
    Err(_) => Ok(0.0),
  }
}
fn deserialize_u32_or_zero<'de, D>(deserializer: D) -> Result<u32, D::Error>
  where D: Deserializer<'de>
{
  let s: String = Deserialize::deserialize(deserializer)?;
  match s.parse() {
    Ok(f) => Ok(f),
    Err(_) => Ok(0),
  }
}

#[derive(Debug, serde::Deserialize, Clone)]
struct TicketData {
  #[serde(rename = "Date")]
  date: String,
  #[serde(rename = "Event")]
  event: String,
  #[serde(rename = "Venue")]
  venue: String,
  #[serde(rename = "Currency")]
  currency: String,
  #[serde(rename = "Ticket")]
  ticket: String,
  #[serde(rename = "Ticket Cost", deserialize_with = "deserialize_f32_or_zero")]
  ticket_cost: f32,
  #[serde(rename = "Total Paid")]
  total_paid: u32,
  #[serde(rename = "Paid (Yesterday)")]
  paid_yesterday: u32,
  #[serde(rename = "Total Inventory", deserialize_with = "deserialize_u32_or_zero")]
  total_inventory: u32,
  #[serde(rename = "Total Face Value")]
  total_face_value: f32,
  #[serde(rename = "Face Value (Yesterday)")]
  face_value_yesterday: f32,
}

#[derive(Debug, Clone)]
struct NormalizedData {
  platinum_vip_2_days: u32,
  platinum_vip_2_days_total: u32,
  vip_admission_2_days: u32,
  vip_admission_2_days_total: u32,
  early_bird_2_days: u32,
  early_bird_2_days_total: u32,
  early_bird_ii_2_days: u32,
  early_bird_ii_2_days_total: u32,
  early_bird_ii_saturday: u32,
  early_bird_ii_saturday_total: u32,
  early_bird_ii_sunday: u32,
  early_bird_ii_sunday_total: u32,
  early_bird_saturday: u32,
  early_bird_saturday_total: u32,
  early_bird_sunday: u32,
  early_bird_sunday_total: u32,
  general_admission_2_days: u32,
  general_admission_2_days_total: u32,
  pit_2_days: u32,
  pit_2_days_total: u32,
  saturday_general_admission: u32,
  saturday_general_admission_total: u32,
  saturday_pit: u32,
  saturday_pit_total: u32,
  sunday_general_admission: u32,
  sunday_general_admission_total: u32,
  sunday_pit: u32,
  sunday_pit_total: u32,
  platinum_vip_2_days_value: f32,
  platinum_vip_2_days_total_value: f32,
  vip_admission_2_days_value: f32,
  vip_admission_2_days_total_value: f32,
  early_bird_2_days_value: f32,
  early_bird_2_days_total_value: f32,
  early_bird_ii_2_days_value: f32,
  early_bird_ii_2_days_total_value: f32,
  early_bird_ii_saturday_value: f32,
  early_bird_ii_saturday_total_value: f32,
  early_bird_ii_sunday_value: f32,
  early_bird_ii_sunday_total_value: f32,
  early_bird_saturday_value: f32,
  early_bird_saturday_total_value: f32,
  early_bird_sunday_value: f32,
  early_bird_sunday_total_value: f32,
  general_admission_2_days_value: f32,
  general_admission_2_days_total_value: f32,
  pit_2_days_value: f32,
  pit_2_days_total_value: f32,
  saturday_general_admission_value: f32,
  saturday_general_admission_total_value: f32,
  saturday_pit_value: f32,
  saturday_pit_total_value: f32,
  sunday_general_admission_value: f32,
  sunday_general_admission_total_value: f32,
  sunday_pit_value: f32,
  sunday_pit_total_value: f32,
}

fn initialize_data() -> NormalizedData {
  return NormalizedData {
    platinum_vip_2_days: 0,
    platinum_vip_2_days_total: 0,
    vip_admission_2_days: 0,
    vip_admission_2_days_total: 0,
    early_bird_2_days: 0,
    early_bird_2_days_total: 0,
    early_bird_ii_2_days: 0,
    early_bird_ii_2_days_total: 0,
    early_bird_ii_saturday: 0,
    early_bird_ii_saturday_total: 0,
    early_bird_ii_sunday: 0,
    early_bird_ii_sunday_total: 0,
    early_bird_saturday: 0,
    early_bird_saturday_total: 0,
    early_bird_sunday: 0,
    early_bird_sunday_total: 0,
    general_admission_2_days: 0,
    general_admission_2_days_total: 0,
    pit_2_days: 0,
    pit_2_days_total: 0,
    saturday_general_admission: 0,
    saturday_general_admission_total: 0,
    saturday_pit: 0,
    saturday_pit_total: 0,
    sunday_general_admission: 0,
    sunday_general_admission_total: 0,
    sunday_pit: 0,
    sunday_pit_total: 0,
    platinum_vip_2_days_value: 0.0,
    platinum_vip_2_days_total_value: 0.0,
    vip_admission_2_days_value: 0.0,
    vip_admission_2_days_total_value: 0.0,
    early_bird_2_days_value: 0.0,
    early_bird_2_days_total_value: 0.0,
    early_bird_ii_2_days_value: 0.0,
    early_bird_ii_2_days_total_value: 0.0,
    early_bird_ii_saturday_value: 0.0,
    early_bird_ii_saturday_total_value: 0.0,
    early_bird_ii_sunday_value: 0.0,
    early_bird_ii_sunday_total_value: 0.0,
    early_bird_saturday_value: 0.0,
    early_bird_saturday_total_value: 0.0,
    early_bird_sunday_value: 0.0,
    early_bird_sunday_total_value: 0.0,
    general_admission_2_days_value: 0.0,
    general_admission_2_days_total_value: 0.0,
    pit_2_days_value: 0.0,
    pit_2_days_total_value: 0.0,
    saturday_general_admission_value: 0.0,
    saturday_general_admission_total_value: 0.0,
    saturday_pit_value: 0.0,
    saturday_pit_total_value: 0.0,
    sunday_general_admission_value: 0.0,
    sunday_general_admission_total_value: 0.0,
    sunday_pit_value: 0.0,
    sunday_pit_total_value: 0.0,
  };
}

fn collect_data(data: &mut NormalizedData, record: TicketData) -> &mut NormalizedData {
  match record.ticket.as_str() {
    "2 DAY PLATINUM VIP" => {
      data.platinum_vip_2_days += record.paid_yesterday;
      data.platinum_vip_2_days_total += record.total_paid;
      data.platinum_vip_2_days_value += record.face_value_yesterday;
      data.platinum_vip_2_days_total_value += record.total_face_value;
    }
    "2 DAY VIP ADMISSION" => {
      data.vip_admission_2_days += record.paid_yesterday;
      data.vip_admission_2_days_total += record.total_paid;
      data.vip_admission_2_days_value += record.face_value_yesterday;
      data.vip_admission_2_days_total_value += record.total_face_value;
    }
    "EARLY BIRD 2 DAYS" => {
      data.early_bird_2_days += record.paid_yesterday;
      data.early_bird_2_days_total += record.total_paid;
      data.early_bird_2_days_value += record.face_value_yesterday;
      data.early_bird_2_days_total_value += record.total_face_value;
    }
    "EARLY BIRD II 2 DAYS" => {
      data.early_bird_ii_2_days += record.paid_yesterday;
      data.early_bird_ii_2_days_total += record.total_paid;
      data.early_bird_ii_2_days_value += record.face_value_yesterday;
      data.early_bird_ii_2_days_total_value += record.total_face_value;
    }
    "Early Bird II Saturday" => {
      data.early_bird_ii_saturday += record.paid_yesterday;
      data.early_bird_ii_saturday_total += record.total_paid;
      data.early_bird_ii_saturday_value += record.face_value_yesterday;
      data.early_bird_ii_saturday_total_value += record.total_face_value;
    }
    "Early Bird II Sunday" => {
      data.early_bird_ii_sunday += record.paid_yesterday;
      data.early_bird_ii_sunday_total += record.total_paid;
      data.early_bird_ii_sunday_value += record.face_value_yesterday;
      data.early_bird_ii_sunday_total_value += record.total_face_value;
    }
    "Early Bird Saturday" => {
      data.early_bird_saturday += record.paid_yesterday;
      data.early_bird_saturday_total += record.total_paid;
      data.early_bird_saturday_value += record.face_value_yesterday;
      data.early_bird_saturday_total_value += record.total_face_value;
    }
    "Early Bird Sunday" => {
      data.early_bird_sunday += record.paid_yesterday;
      data.early_bird_sunday_total += record.total_paid;
      data.early_bird_sunday_value += record.face_value_yesterday;
      data.early_bird_sunday_total_value += record.total_face_value;
    }
    "General Admission 2 DAYS" => {
      data.general_admission_2_days += record.paid_yesterday;
      data.general_admission_2_days_total += record.total_paid;
      data.general_admission_2_days_value += record.face_value_yesterday;
      data.general_admission_2_days_total_value += record.total_face_value;
    }
    "Pit 2 DAYS" => {
      data.pit_2_days += record.paid_yesterday;
      data.pit_2_days_total += record.total_paid;
      data.pit_2_days_value += record.face_value_yesterday;
      data.pit_2_days_total_value += record.total_face_value;
    }
    "Saturday General Admission" => {
      data.saturday_general_admission += record.paid_yesterday;
      data.saturday_general_admission_total += record.total_paid;
      data.saturday_general_admission_value += record.face_value_yesterday;
      data.saturday_general_admission_total_value += record.total_face_value;
    }
    "Saturday Pit" => {
      data.saturday_pit += record.paid_yesterday;
      data.saturday_pit_total += record.total_paid;
      data.saturday_pit_value += record.face_value_yesterday;
      data.saturday_pit_total_value += record.total_face_value;
    }
    "Sunday General Admission" => {
      data.sunday_general_admission += record.paid_yesterday;
      data.sunday_general_admission_total += record.total_paid;
      data.sunday_general_admission_value += record.face_value_yesterday;
      data.sunday_general_admission_total_value += record.total_face_value;
    }
    "Sunday Pit" => {
      data.sunday_pit += record.paid_yesterday;
      data.sunday_pit_total += record.total_paid;
      data.sunday_pit_value += record.face_value_yesterday;
      data.sunday_pit_total_value += record.total_face_value;
    }
    _ => (),
  }
  data
}

pub fn process_tickets() -> Result<(), Box<dyn Error>> {
  // 1. File Discovery (replace with your actual file pattern)
  let file_pattern = "src/csvs/*.csv";

  let mut file_paths = glob(file_pattern)?.filter_map(Result::ok).collect::<Vec<_>>();
  file_paths.sort();

  // // 2. Data Accumulation
  let mut accumulated_data: BTreeMap<String, NormalizedData> = BTreeMap::new();

  for file_path in file_paths {
    let date: String = file_path.to_str().unwrap().chars().skip(9).take(10).collect();

    let file = File::open(&file_path)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(BufReader::new(file));

    for result in reader.deserialize() {
      // Take all data from one file and accumulate it to a row in the output CSV
      let record: TicketData = result?;
      let entry = accumulated_data.entry(date.to_string());
      if let std::collections::btree_map::Entry::Occupied(mut e) = entry {
        let data = e.get_mut();
        collect_data(data, record.clone());
      } else {
        let data = initialize_data();
        accumulated_data.insert(date.to_string(), data);
      }
    }
  }

  // 3. CSV Writing
  let output_file = File::create("src/output/combined_ticket_sales.csv")?;
  let mut writer = WriterBuilder::new().has_headers(true).from_writer(BufWriter::new(output_file));

  writer.write_record(
    &[
      "Date",
      "2 Day Platinum",
      "2 Day VIP",
      "Gen 2 Days",
      "Gen Saturday",
      "Gen Sunday",
      "Pit 2 Days",
      "Pit Saturday",
      "Pit Sunday",
      "EB II 2 Days",
      "EB II Saturday",
      "EB II Sunday",
      "EB 2 Days",
      "EB Saturday",
      "EB Sunday",
      "2 Day Platinum Value",
      "2 Day VIP Value",
      "Gen 2 Days Value",
      "Gen Saturday Value",
      "Gen Sunday Value",
      "Pit 2 Days Value",
      "Pit Saturday Value",
      "Pit Sunday Value",
      "EB II 2 Days Value",
      "EB II Saturday Value",
      "EB II Sunday Value",
      "EB 2 Days Value",
      "EB Saturday Value",
      "EB Sunday Value",
      "2 Day Platinum Total",
      "2 Day VIP Total",
      "Gen 2 Days Total",
      "Gen Saturday Total",
      "Gen Sunday Total",
      "Pit 2 Days Total",
      "Pit Saturday Total",
      "Pit Sunday Total",
      "EB II 2 Days Total",
      "EB II Saturday Total",
      "EB II Sunday Total",
      "EB 2 Days Total",
      "EB Saturday Total",
      "EB Sunday Total",
      "2 Day Platinum Total Value",
      "2 Day VIP Total Value",
      "Gen 2 Days Total Value",
      "Gen Saturday Total Value",
      "Gen Sunday Total Value",
      "Pit 2 Days Total Value",
      "Pit Saturday Total Value",
      "Pit Sunday Total Value",
      "EB II 2 Days Total Value",
      "EB II Saturday Total Value",
      "EB II Sunday Total Value",
      "EB 2 Days Total Value",
      "EB Saturday Total Value",
      "EB Sunday Total Value",
    ]
  )?;

  fn empty_string_if_zero_u32(value: u32) -> String {
    if value == 0 { String::new() } else { value.to_string() }
  }
  fn empty_string_if_zero_f32(value: f32) -> String {
    if value == 0.0 { String::new() } else { value.to_string() }
  }

  for (date, data) in accumulated_data {
    writer.write_record(
      &[
        date.to_string(),
        empty_string_if_zero_u32(data.platinum_vip_2_days),
        empty_string_if_zero_u32(data.vip_admission_2_days),
        empty_string_if_zero_u32(data.general_admission_2_days),
        empty_string_if_zero_u32(data.saturday_general_admission),
        empty_string_if_zero_u32(data.sunday_general_admission),
        empty_string_if_zero_u32(data.pit_2_days),
        empty_string_if_zero_u32(data.saturday_pit),
        empty_string_if_zero_u32(data.sunday_pit),
        empty_string_if_zero_u32(data.early_bird_ii_2_days),
        empty_string_if_zero_u32(data.early_bird_ii_saturday),
        empty_string_if_zero_u32(data.early_bird_ii_sunday),
        empty_string_if_zero_u32(data.early_bird_2_days),
        empty_string_if_zero_u32(data.early_bird_saturday),
        empty_string_if_zero_u32(data.early_bird_sunday),
        empty_string_if_zero_f32(data.platinum_vip_2_days_value),
        empty_string_if_zero_f32(data.vip_admission_2_days_value),
        empty_string_if_zero_f32(data.general_admission_2_days_value),
        empty_string_if_zero_f32(data.saturday_general_admission_value),
        empty_string_if_zero_f32(data.sunday_general_admission_value),
        empty_string_if_zero_f32(data.pit_2_days_value),
        empty_string_if_zero_f32(data.saturday_pit_value),
        empty_string_if_zero_f32(data.sunday_pit_value),
        empty_string_if_zero_f32(data.early_bird_ii_2_days_value),
        empty_string_if_zero_f32(data.early_bird_ii_saturday_value),
        empty_string_if_zero_f32(data.early_bird_ii_sunday_value),
        empty_string_if_zero_f32(data.early_bird_2_days_value),
        empty_string_if_zero_f32(data.early_bird_saturday_value),
        empty_string_if_zero_f32(data.early_bird_sunday_value),
        empty_string_if_zero_u32(data.platinum_vip_2_days_total),
        empty_string_if_zero_u32(data.vip_admission_2_days_total),
        empty_string_if_zero_u32(data.general_admission_2_days_total),
        empty_string_if_zero_u32(data.saturday_general_admission_total),
        empty_string_if_zero_u32(data.sunday_general_admission_total),
        empty_string_if_zero_u32(data.pit_2_days_total),
        empty_string_if_zero_u32(data.saturday_pit_total),
        empty_string_if_zero_u32(data.sunday_pit_total),
        empty_string_if_zero_u32(data.early_bird_ii_2_days_total),
        empty_string_if_zero_u32(data.early_bird_ii_saturday_total),
        empty_string_if_zero_u32(data.early_bird_ii_sunday_total),
        empty_string_if_zero_u32(data.early_bird_2_days_total),
        empty_string_if_zero_u32(data.early_bird_saturday_total),
        empty_string_if_zero_u32(data.early_bird_sunday_total),
        empty_string_if_zero_f32(data.platinum_vip_2_days_total_value),
        empty_string_if_zero_f32(data.vip_admission_2_days_total_value),
        empty_string_if_zero_f32(data.general_admission_2_days_total_value),
        empty_string_if_zero_f32(data.saturday_general_admission_total_value),
        empty_string_if_zero_f32(data.sunday_general_admission_total_value),
        empty_string_if_zero_f32(data.pit_2_days_total_value),
        empty_string_if_zero_f32(data.saturday_pit_total_value),
        empty_string_if_zero_f32(data.sunday_pit_total_value),
        empty_string_if_zero_f32(data.early_bird_ii_2_days_total_value),
        empty_string_if_zero_f32(data.early_bird_ii_saturday_total_value),
        empty_string_if_zero_f32(data.early_bird_ii_sunday_total_value),
        empty_string_if_zero_f32(data.early_bird_2_days_total_value),
        empty_string_if_zero_f32(data.early_bird_saturday_total_value),
        empty_string_if_zero_f32(data.early_bird_sunday_total_value),
      ]
    )?;
  }

  writer.flush()?;
  Ok(())
}
