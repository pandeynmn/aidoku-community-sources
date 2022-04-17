use aidoku::{
	error::Result, std::String, std::Vec, std::ObjectRef, std::html::Node,
	Manga, MangaStatus, MangaContentRating, MangaViewer, Chapter,
};

use super::helper::{chapter_url_encode, chapter_image};

// Parse manga with title and cover
pub fn parse_basic_manga(manga_object: ObjectRef) -> Result<Manga> {
	let id = manga_object.get("i").as_string()?.read();
	let title = manga_object.get("s").as_string()?.read();

	let mut cover = String::from("https://cover.nep.li/cover/");
	cover.push_str(&id);
	cover.push_str(".jpg");

	Ok(Manga {
		id,
		cover,
		title,
		author: String::new(),
		artist: String::new(),
		description: String::new(),
		url: String::new(),
		categories: Vec::new(),
		status: MangaStatus::Unknown,
		nsfw: MangaContentRating::Safe,
		viewer: MangaViewer::Default,
	})
}

// Parse complete manga info
pub fn parse_full_manga(id: String, url: String, manga_node: Node) -> Result<Manga> {
	let cover = manga_node.select("div.BoxBody > div.row img").attr("src").read();
	let title = manga_node.select("div.BoxBody > div.row h1").text().read();
	let author = manga_node.select("div.BoxBody > div.row li.list-group-item:has(span:contains(Author)) a")
					.first()
					.text()
					.read();
	let description = manga_node.select("div.BoxBody > div.row div.Content").text().read();

	let mut categories: Vec<String> = Vec::new();
	manga_node.select("li.list-group-item:has(span:contains(Genre)) a")
				.array()
				.for_each(|tag| categories.push(tag.as_node().text().read()));

	let status = match manga_node.select("div.BoxBody > div.row li.list-group-item:has(span:contains(Status)) a:contains(Scan)").first().text().read().as_str() {
		"Ongoing (Scan)" => MangaStatus::Ongoing,
		"Complete (Scan)" => MangaStatus::Completed,
		"Hiatus (Scan)" => MangaStatus::Hiatus,
		"Cancelled (Scan)" => MangaStatus::Cancelled,
		"Discontinued (Scan)" => MangaStatus::Cancelled,
		_ => MangaStatus::Unknown,
	};

	let nsfw = if categories.iter().any(|e| e == "Adult" || e == "Hentai" || e == "Mature") {
		MangaContentRating::Nsfw
	} else if categories.iter().any(|e| e == "Ecchi") {
		MangaContentRating::Suggestive
	} else {
		MangaContentRating::Safe
	};

	let viewer = match manga_node.select("li.list-group-item:has(span:contains(Type)) a").first().text().read().as_str() {
		"Manhua" => MangaViewer::Scroll,
		"Manhwa" => MangaViewer::Scroll,
		_ => MangaViewer::Rtl,
	};

	Ok(Manga {
		id,
		cover,
		title,
		author,
		artist: String::new(),
		description,
		url,
		categories,
		status,
		nsfw,
		viewer,
	})
}

// Parse chapter info
pub fn parse_chapter(manga_id: &str, chapter_object: ObjectRef) -> Result<Chapter> {
	let id = chapter_object.get("Chapter").as_string()?.read();

	let mut path = String::from(manga_id);
	path.push_str(&chapter_url_encode(&id));

	let chapter = id[1..].parse::<f32>().unwrap_or(-10.0) / 10.0;

	let mut title = match chapter_object.get("ChapterName").as_string() {
		Ok(title) => title.read(),
		Err(_) => String::new(),
	};
	if title.is_empty() {
		title = chapter_object.get("Type").as_string()?.read();
		title.push_str(" ");
		title.push_str(&chapter_image(&id, false));
	}

	let date_updated = chapter_object.get("Date").as_date("yyyy-MM-dd HH:mm:SS").unwrap_or(-1.0);

	let mut url = String::from("https://mangasee123.com/read-online/");
	url.push_str(&path);

	Ok(Chapter {
		id: path,
		title,
		volume: -1.0,
		chapter,
		date_updated,
		scanlator: String::new(),
		url,
		lang: String::from("en"),
	})
}
