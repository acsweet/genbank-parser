use arrow::array::StringArray;
use arrow::datatypes::{DataType, Field, Schema};
use arrow::ipc::writer::FileWriter;
use arrow::record_batch::RecordBatch;
use crate::{Sequence, Protein};
use std::fs::File;
use std::sync::Arc;

pub trait ArrowVec {
    fn create_schema() -> Schema;
    fn create_record_batch(&self, schema: Schema) -> RecordBatch;
}

pub fn write_arrow_file<T: ArrowVec>(
    records: &T,
    output_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let schema = T::create_schema();
    let record_batch = records.create_record_batch(schema.clone());

    let file = File::create(output_path)?;
    let mut writer = FileWriter::try_new(file, &schema)?;
    writer.write(&record_batch)?;
    writer.finish()?;

    Ok(())
}

fn to_string_array(data: &[&Vec<u8>]) -> Arc<dyn arrow::array::Array> {
    let array = StringArray::from_iter_values(data.iter().map(|v| std::str::from_utf8(v).unwrap()));
    Arc::new(array)
}

impl ArrowVec for Vec<Sequence> {
    fn create_schema() -> Schema {
        let fields = vec![
            Field::new("version", DataType::Utf8, false),
            Field::new("definition", DataType::Utf8, false),
            Field::new("organism", DataType::Utf8, false),
            Field::new("taxonomy", DataType::Utf8, false),
            Field::new("sequence", DataType::Utf8, false),
            Field::new("host", DataType::Utf8, false),
            Field::new("mol_type", DataType::Utf8, false),
        ];
        Schema::new(fields)
    }

    fn create_record_batch(&self, schema: Schema) -> RecordBatch {
        let version = to_string_array(&self.iter().map(|s| &s.version).collect::<Vec<_>>());
        let definition = to_string_array(&self.iter().map(|s| &s.definition).collect::<Vec<_>>());
        let organism = to_string_array(&self.iter().map(|s| &s.organism).collect::<Vec<_>>());
        let taxonomy = to_string_array(&self.iter().map(|s| &s.taxonomy).collect::<Vec<_>>());
        let sequence = to_string_array(&self.iter().map(|s| &s.sequence).collect::<Vec<_>>());
        let host = to_string_array(&self.iter().map(|s| &s.host).collect::<Vec<_>>());
        let mol_type = to_string_array(&self.iter().map(|s| &s.mol_type).collect::<Vec<_>>());

        let columns = vec![
            Arc::new(version) as Arc<dyn arrow::array::Array>,
            Arc::new(definition),
            Arc::new(organism),
            Arc::new(taxonomy),
            Arc::new(sequence),
            Arc::new(host),
            Arc::new(mol_type),
        ];
        RecordBatch::try_new(Arc::new(schema), columns).unwrap()
    }
}

impl ArrowVec for Vec<Protein> {
    fn create_schema() -> Schema {
        let fields = vec![
            Field::new("protein_id", DataType::Utf8, false),
            Field::new("source_id", DataType::Utf8, false),
            Field::new("sequence", DataType::Utf8, false),
            Field::new("location", DataType::Utf8, false),
        ];
        Schema::new(fields)
    }

    fn create_record_batch(&self, schema: Schema) -> RecordBatch {
        let protein_id = to_string_array(&self.iter().map(|s| &s.protein_id).collect::<Vec<_>>());
        let source_id = to_string_array(&self.iter().map(|s| &s.source_id).collect::<Vec<_>>());
        let sequence = to_string_array(&self.iter().map(|s| &s.sequence).collect::<Vec<_>>());
        let location = to_string_array(&self.iter().map(|s| &s.location).collect::<Vec<_>>());

        let columns = vec![
            Arc::new(protein_id) as Arc<dyn arrow::array::Array>,
            Arc::new(source_id),
            Arc::new(sequence),
            Arc::new(location),
        ];
        RecordBatch::try_new(Arc::new(schema), columns).unwrap()
    }
}