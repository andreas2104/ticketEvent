
#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Ticket {
  id: String,
  event_id: String,
  serial_number: String,
  is_paid: bool,
  is_scanned: bool,
  scanned_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Template {
  id: String,
  image_path: String,
  qr_x: f64,
  qr_y: f64,
  num_x: f64,
  num_y: f64,
  num_width: f64,
  num_height: f64,
  num_padding: f64,
  num_border_radius: f64,
  num_border_color: String,
  num_border_width: f64,
  num_border_style: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Event {
  id: String,
  name: String,
  description: String,
  start_time: String,
  end_time: String,
  location: String,
  capacity: u32,
  price: f64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct User {
  id: String,
  email: String,
  name: String,
  phone: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Order {
  id: String,
  user_id: String,
  event_id: String,
  ticket_id: String,
  quantity: u32,
  total_price: f64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Ticket {
  id: String,
  event_id: String,
  serial_number: String,
  is_paid: bool,
  is_scanned: bool,
  scanned_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Template {
  id: String,
  image_path: String,
  qr_x: f64,
  qr_y: f64,
  num_x: f64,
  num_y: f64,
  num_width: f64,
  num_height: f64,
  num_padding: f64,
  num_border_radius: f64,
  num_border_color: String,
  num_border_width: f64,
  num_border_style: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Event {
  id: String,
  name: String,
  description: String,
  start_time: String,
  end_time: String,
  location: String,
  capacity: u32,
  price: f64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct User {
  id: String,
  email: String,
  name: String,
  phone: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Order {
  id: String,
  user_id: String,
  event_id: String,
  ticket_id: String,
  quantity: u32,
  total_price: f64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Ticket {
  id: String,
  event_id: String,
  serial_number: String,
  is_paid: bool,
  is_scanned: bool,
  scanned_at: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Template {
  id: String,
  image_path: String,
  qr_x: f64,
  qr_y: f64,
  num_x: f64,
  num_y: f64,
  num_width: f64,
  num_height: f64,
  num_padding: f64,
  num_border_radius: f64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Event {
  id: String,
  name: String,
  description: String,
  start_time: String,
  end_time: String,
  location: String,
  capacity: u32,
  price: f64,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct User {
  id: String,
  email: String,
  name: String,
  phone: String,
}

#[tokio::main]
