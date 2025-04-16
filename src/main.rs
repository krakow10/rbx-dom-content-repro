use rbx_dom_weak::Instance;
use std::collections::HashSet;

fn get_content_url(variant:&rbx_dom_weak::types::Variant)->Option<&str>{
	match variant{
		rbx_dom_weak::types::Variant::Content(content_id)=>Some(content_id.as_ref()),
		_=>None,
	}
}
fn accumulate_content_id<'a>(content_list:&mut HashSet<&'a str>,object:&'a Instance,property:&str){
	let Some(content)=object.properties.get(property).and_then(get_content_url) else{
		println!("property={} does not exist for class={}",property,object.class.as_str());
		return;
	};
	content_list.insert(content);
}
#[derive(Default)]
struct UniqueAssets<'a>{
	meshes:HashSet<&'a str>,
	unions:HashSet<&'a str>,
	textures:HashSet<&'a str>,
}
impl<'a> UniqueAssets<'a>{
	fn collect(&mut self,object:&'a Instance){
		match object.class.as_str(){
			"Beam"=>accumulate_content_id(&mut self.textures,object,"Texture"),
			"Decal"=>accumulate_content_id(&mut self.textures,object,"Texture"),
			"Texture"=>accumulate_content_id(&mut self.textures,object,"Texture"),
			"FileMesh"=>accumulate_content_id(&mut self.textures,object,"TextureId"),
			"MeshPart"=>{
				accumulate_content_id(&mut self.textures,object,"TextureID");
				accumulate_content_id(&mut self.meshes,object,"MeshId");
			},
			"SpecialMesh"=>accumulate_content_id(&mut self.meshes,object,"MeshId"),
			"ParticleEmitter"=>accumulate_content_id(&mut self.textures,object,"Texture"),
			"Sky"=>{
				accumulate_content_id(&mut self.textures,object,"MoonTextureId");
				accumulate_content_id(&mut self.textures,object,"SkyboxBk");
				accumulate_content_id(&mut self.textures,object,"SkyboxDn");
				accumulate_content_id(&mut self.textures,object,"SkyboxFt");
				accumulate_content_id(&mut self.textures,object,"SkyboxLf");
				accumulate_content_id(&mut self.textures,object,"SkyboxRt");
				accumulate_content_id(&mut self.textures,object,"SkyboxUp");
				accumulate_content_id(&mut self.textures,object,"SunTextureId");
			},
			"UnionOperation"=>accumulate_content_id(&mut self.unions,object,"AssetId"),
			_=>(),
		}
	}
}

fn main(){
	// let path=std::env::args().skip(1).next().expect("Expected 1 argument");
	let path="5692139328.rbxm";
	// read entire file
	let mut assets=UniqueAssets::default();
	let data=std::fs::read(path).expect("IO error");
	let dom=rbx_binary::from_reader(std::io::Cursor::new(data)).expect("rbx_binary error");
	for object in dom.descendants(){
		assets.collect(object);
	}
	println!("num collected meshes={}",assets.meshes.len());
	println!("num collected unions={}",assets.unions.len());
	println!("num collected textures={}",assets.textures.len());
}
