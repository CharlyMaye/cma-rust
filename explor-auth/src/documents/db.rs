use mongodb::{Database, bson::doc, options::IndexOptions, IndexModel};

pub async fn initialize_documents_collection(database: &Database) -> Result<(), mongodb::error::Error> {
    let collection_names = database.list_collection_names().await?;
    
    // Vérifier si la collection documents existe déjà
    if !collection_names.contains(&"documents".to_string()) {
        create_documents_collection(database).await?;
    }
    
    Ok(())
}

async fn create_documents_collection(database: &Database) -> Result<(), mongodb::error::Error> {
    println!("📝 Creating collection: documents");
    database.create_collection("documents").await?;
    
    // Créer un index unique sur doc_id
    let documents_collection = database.collection::<mongodb::bson::Document>("documents");
    let index_options = IndexOptions::builder().unique(true).build();
    let index_model = IndexModel::builder()
        .keys(doc! { "doc_id": 1 })
        .options(index_options)
        .build();
    
    documents_collection.create_index(index_model).await?;
    println!("📌 Index unique créé sur 'doc_id'");
    
    Ok(())
}
