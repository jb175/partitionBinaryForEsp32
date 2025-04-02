use esp_idf_part::PartitionTable;

fn main() {
    let csv = std::fs::read_to_string("partitions.csv").unwrap();
    let table = PartitionTable::try_from_str(csv).unwrap();
    std::fs::write("partitions.bin", table.to_bin().unwrap()).unwrap();
}
