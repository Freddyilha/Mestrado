use rand::prelude::*;

// https://www.techtarget.com/searchstorage/answer/RAID-types-and-benefits-explained

fn main() {
    let mut raid_0 = match RaidFactory::create(RaidType::Zero, None) {
        Ok(raid) => raid,
        Err(e) => {
            eprintln!("Failed to create RAID: {}", e);
            return;
        }
    };

    let disk_values = generate_data(20);
    let mid = disk_values.len() / 2;
    raid_0.get_disks_mut()[0].extend(&disk_values[0..mid]);
    raid_0.get_disks_mut()[1].extend(&disk_values[mid..]);

    println!("------------- RAID ZERO -------------");
    raid_0.print_data(0);
    raid_0.print_data(1);

    raid_0.corrupt_data();

    raid_0.print_data(0);
    raid_0.print_data(1);

    let mut raid_1 = match RaidFactory::create(RaidType::One, None) {
        Ok(raid) => raid,
        Err(e) => {
            eprintln!("Failed to create RAID: {}", e);
            return;
        }
    };

    raid_1.get_disks_mut()[0].extend(&disk_values[0..mid]);

    let disk_0_copy = raid_1.get_disk(0).clone();
    raid_1.get_disks_mut()[1].extend(&disk_0_copy);

    println!("------------- RAID ONE -------------");
    raid_1.print_data(0);
    raid_1.print_data(1);

    raid_1.corrupt_data();

    raid_1.print_data(0);
    raid_1.print_data(1);

    let mut raid_5 = match RaidFactory::create(RaidType::Five, Some(3)) {
        Ok(raid) => raid,
        Err(e) => {
            eprintln!("Failed to create RAID: {}", e);
            return;
        }
    };

    raid_5.get_disks_mut()[0].extend(&disk_values[0..mid]);

    let disk_0_copy = raid_5.get_disk(0).clone();
    raid_5.get_disks_mut()[1].extend(&disk_0_copy);

    println!("------------- RAID FIVE -------------");
    raid_5.print_data(0);
    raid_5.print_data(1);

    raid_5.corrupt_data();

    raid_5.print_data(0);
    raid_5.print_data(1);
}

trait DataStructure {
    fn print_data(&self, disk_number: usize);
    fn get_disks_mut(&mut self) -> &mut Vec<Vec<u16>>;
    fn get_parity_disks_mut(&mut self) -> &mut Vec<Vec<u16>>;
    fn get_disk(&self, disk: usize) -> &Vec<u16>;
    fn corrupt_data(&mut self) -> &mut Vec<Vec<u16>>;
    fn create_parity_disk(&mut self) -> &mut Vec<Vec<u16>>;
}

struct RaidData {
    raid_type: RaidType,
    disks: Vec<Vec<u16>>,
    parity_disks: Vec<Vec<u16>>,
}

impl RaidData {
    const MIN_DISKS: usize = 2;

    fn new(raid_type: RaidType, disks: u8) -> Result<Self, String> {
        if (disks as usize) < Self::MIN_DISKS {
            return Err(format!(
                "RAID requires at least {} disks, got {}",
                Self::MIN_DISKS,
                disks
            ));
        }
        Ok(RaidData {
            raid_type,
            disks: vec![Vec::new(); disks as usize],
            parity_disks: vec![Vec::new(); disks as usize],
        })
    }
}

impl DataStructure for RaidData {
    fn print_data(&self, disk_number: usize) {
        if let Some(disk) = self.disks.get(disk_number) {
            println!("Disk {}:{:?}", disk_number, disk);
        }
    }

    fn get_disks_mut(&mut self) -> &mut Vec<Vec<u16>> {
        &mut self.disks
    }

    fn get_parity_disks_mut(&mut self) -> &mut Vec<Vec<u16>> {
        &mut self.parity_disks
    }

    fn get_disk(&self, disk: usize) -> &Vec<u16> {
        &self.disks[disk]
    }

    fn corrupt_data(&mut self) -> &mut Vec<Vec<u16>> {
        let disks = (self.disks.len()) as u16;
        let random_disk: usize = rand::random_range(0..disks) as usize;

        if let Some(disk) = self.disks.get_mut(random_disk) {
            let disk_size = disk.len();
            if disk_size > 0 {
                let random_position = rand::random_range(0..disk_size);
                disk[random_position] = 0;
            }
        }

        &mut self.disks
    }

    fn create_parity_disk(&mut self) -> &mut Vec<Vec<u16>> {
        &mut self.parity_disks
    }
}

#[derive(Clone, Copy)]
enum RaidType {
    Zero,
    One,
    Five,
}

struct RaidFactory;

fn generate_data(amount: u16) -> Vec<u16> {
    let mut rng = rand::rng();
    let mut nums: Vec<u16> = (1..amount).collect();
    nums.shuffle(&mut rng);

    nums
}

impl RaidFactory {
    fn create(raid_type: RaidType, num_disks: Option<u8>) -> Result<Box<dyn DataStructure>, String> {
        let disk_count = num_disks.unwrap_or(2);
        Ok(Box::new(RaidData::new(raid_type, disk_count)?))
    }
}
