use serde_derive::Deserialize;


#[derive(Deserialize, Clone, Debug)]
pub struct Config 
{
	pub
	  package : Package,
	pub
	  tools : Option<Tools>,
	pub
	  meta : Option<Meta>,
	pub
	  compile : Option<Compile>,
	pub
	  linking : Option<Linking>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Package 
{
	pub
	  name : String,
	pub
	  version : String,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Tools 
{
	pub
	  compiler : Option<String>,
	pub
	  linker : Option<String>,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Meta 
{
	pub
	  src_files : Option<String>,
	pub
	  header_dirs : Option<String>,
	pub
	  obj_files : Option<String>,
	pub
	  compiler_args : Option<String>,
	pub
	  linker_args : Option<String>,
	pub
	  output_name : Option<String>,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Compile 
{
	pub
	  before : Option<Vec<String>>,
	pub
	  run : Option<Vec<String>>,
	pub
	  after : Option<Vec<String>>,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct Linking 
{
	pub
	  before : Option<Vec<String>>,
	pub
	  run : Option<Vec<String>>,
	pub
	  after : Option<Vec<String>>,
}
