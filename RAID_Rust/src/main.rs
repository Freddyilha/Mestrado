use rand::prelude::*;

fn main() {
    let mut raid_0 = match RaidFactory::create(RaidType::Zero) {
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

    raid_0.print_data(0);
    raid_0.print_data(1);

    raid_0.corrupt_data();

    raid_0.print_data(0);
    raid_0.print_data(1);
}

trait DataStructure {
    fn print_data(&self, disk: usize);
    fn get_disks_mut(&mut self) -> &mut Vec<Vec<u16>>;
    fn corrupt_data(&mut self) -> &mut Vec<Vec<u16>>;
}

struct Zero {
    disks: Vec<Vec<u16>>,
}

impl Zero {
    const MIN_DISKS: usize = 2;

    fn new(disks: u8) -> Result<Self, String> {
        if (disks as usize) < Self::MIN_DISKS {
            return Err(format!(
                "RAID 0 requires at least {} disks, got {}",
                Self::MIN_DISKS,
                disks
            ));
        }
        Ok(Zero {
            disks: vec![Vec::new(); disks as usize],
        })
    }
}

impl DataStructure for Zero {
    fn print_data(&self, disk: usize) {
        if let Some(disk) = self.disks.get(disk) {
            println!("{:?}", disk);
        }
    }

    fn get_disks_mut(&mut self) -> &mut Vec<Vec<u16>> {
        &mut self.disks
    }

    fn corrupt_data(&mut self) -> &mut Vec<Vec<u16>> {
        let disks = (self.disks.len()) as u16;
        let random_disk: usize = rand::random_range(0..disks) as usize;
        println!("random disk: {:?}", random_disk);

        if let Some(disk) = self.disks.get_mut(random_disk) {
            let disk_size = disk.len();
            let random_position = rand::random_range(0..disk_size);
            println!("random position: {:?}", random_position);

            disk[random_position] = 0;
        }

        &mut self.disks
    }
}

enum RaidType {
    Zero,
}

struct RaidFactory;

fn generate_data(amount: u16) -> Vec<u16> {
    let mut rng = rand::rng();
    let mut nums: Vec<u16> = (1..amount).collect();
    nums.shuffle(&mut rng);

    nums
}

impl RaidFactory {
    fn create(raid_type: RaidType) -> Result<Box<dyn DataStructure>, String> {
        match raid_type {
            RaidType::Zero => Ok(Box::new(Zero::new(2)?)),
        }
    }
}
