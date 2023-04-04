trait DeviceInfoProvider {
    fn get_device_title(&self) -> String;
}

struct Device {
    title: String,
}

impl DeviceInfoProvider for Device {
    fn get_device_title(&self) -> String {
        let title = &self.title;
        title.clone()
    }
}

// Помещение имеет уникальное название и содержит названия нескольких устройств.
struct Room<'a> {
    title: String,                // название
    devices: &'a Vec<&'a Device>, // название нескольких устройств
}

impl<'a> Room<'a> {
    fn new(title: String, devices: &'a Vec<&'a Device>) -> Self {
        Room { title, devices }
    }
}

struct House<'a> {
    title: String,
    rooms: &'a Vec<&'a Room<'a>>,
}

impl<'a> House<'a> {
    fn new(title: String, rooms: &'a Vec<&'a Room>) -> Self {
        House { title, rooms }
    }

    fn get_rooms(&self) -> &Vec<&Room> {
        let rooms = self.rooms;
        rooms
    }

    // Библиотека позволяет получать список устройств в помещении.
    fn get_devices_in_room(&self, find_room_title: String) -> &Vec<&Device> {
        let house_rooms = self.rooms;

        for i in 0..house_rooms.len() {
            let current_room = house_rooms[i];

            if current_room.title == find_room_title {
                return current_room.devices;
            }
        }

        panic!();
    }

    // Эта функция принимает в качестве аргумента обобщённый тип, позволяющий получить текстовую информацию
    // о состоянии устройства, для включения в отчёт.
    fn get_report_about_device(&self, device: &impl DeviceInfoProvider) -> String {
        let rooms = self.rooms;

        for i in 0..rooms.len() {
            let curr_room = rooms[i];

            let devices = curr_room.devices;

            for j in 0..devices.len() {
                let curr_device = devices[j];

                if curr_device.get_device_title() == device.get_device_title() {
                    let report = format!(
                        "Устройство: {}, находится в доме: {}, в комнате: {}",
                        device.get_device_title(),
                        &self.title,
                        curr_room.title
                    );
                    return report;
                }
            }
        }

        panic!();
    }
}

fn main() {
    let device_1 = Device {
        title: String::from("device1"),
    };
    let device_2 = Device {
        title: String::from("device2"),
    };
    let device_3 = Device {
        title: String::from("device3"),
    };
    let device_4 = Device {
        title: String::from("device4"),
    };

    let devices_for_room_1 = vec![&device_1, &device_2];
    let devices_for_room_2 = vec![&device_3, &device_4];

    let room1 = Room::new(String::from("room1"), &devices_for_room_1);

    let room2 = Room::new(String::from("room2"), &devices_for_room_2);

    let rooms = vec![&room1, &room2];

    // Дом имеет название и содержит несколько помещений.
    let house = House::new(String::from("house1"), &rooms);

    // Библиотека позволяет запросить список помещений в доме.
    let house_rooms = house.get_rooms();
    for i in 0..house_rooms.len() {
        let room = house_rooms[i];
        println!("room title: {}", room.title);
    }

    let devices_in_room_1 = house.get_devices_in_room(String::from("room1"));
    for i in 0..devices_in_room_1.len() {
        let device = &devices_in_room_1[i];
        println!("device in room1: {}", device.get_device_title());
    }

    // Если помещение не найдено выбрасываем ошибку
    // let devices_in_room_3 = house.get_devices_in_room(String::from("room3"));

    let report_about_device_1 = house.get_report_about_device(&device_1);
    println!("{}", report_about_device_1);

    let report_about_device_4 = house.get_report_about_device(&device_4);
    println!("{}", report_about_device_4);
}
