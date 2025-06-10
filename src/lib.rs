use worker::*;
use console_error_panic_hook::set_once as set_panic_hook;
use serde::Deserialize;

#[derive(Deserialize)]
struct RequestBody {
    url: Option<String>,
    svg: Option<String>,
}

#[event(fetch)]
pub async fn main(mut req: Request, _env: Env, _ctx: worker::Context) -> Result<Response> {
    console_log!("{} - [{}]", Date::now().to_string(), req.path());
    set_panic_hook();
    
    match req.method() {
        Method::Get => {
            let image_path = req.path()[1..].to_string();
            match handle_render_from_url(image_path).await {
                Err(err) => {
                    println!("error: {:?}", err);
                    Response::error(format!("an unexpected error occurred: {}", err), 500)
                }
                Ok(res) => Ok(res),
            }
        }
        Method::Post => {
            let body = req.json::<RequestBody>().await?;
            
            // 检查是提供了URL还是直接的SVG内容
            if let Some(url) = body.url {
                match handle_render_from_url(url).await {
                    Err(err) => {
                        println!("error: {:?}", err);
                        Response::error(format!("an unexpected error occurred: {}", err), 500)
                    }
                    Ok(res) => Ok(res),
                }
            } else if let Some(svg_content) = body.svg {
                match handle_render_from_content(svg_content).await {
                    Err(err) => {
                        println!("error: {:?}", err);
                        Response::error(format!("an unexpected error occurred: {}", err), 500)
                    }
                    Ok(res) => Ok(res),
                }
            } else {
                Response::error("Either 'url' or 'svg' field must be provided in request body", 400)
            }
        }
        _ => Response::error("Method not allowed", 405),
    }
}

// 从URL获取SVG并渲染
async fn handle_render_from_url(svg_url: String) -> Result<Response> {
    console_log!("svgUrl: {}", svg_url);
    
    let url = Url::parse(&svg_url)
        .map_err(|err| format!("failed to parse URL: {}", err))?;
    
    let mut res = Fetch::Url(url)
        .send()
        .await
        .map_err(|err| format!("failed to request remote image: {}", err))?;
    
    if res.status_code() != 200 {
        let body = res.text().await?;
        return Response::error(
            format!("upstream image returned: {}: {}", res.status_code(), body),
            500,
        );
    }
    
    let svg_data = res.bytes().await?;
    render_svg_to_png(svg_data).await
}

// 直接从SVG内容渲染
async fn handle_render_from_content(svg_content: String) -> Result<Response> {
    console_log!("rendering SVG from content, length: {}", svg_content.len());

    let header = r#"<?xml
version="1.0" encoding="UTF-8" standalone="no"?>
<!DOCTYPE svg PUBLIC "-//W3C//DTD SVG 1.1//EN" "http://www.w3.org/Graphics/SVG/1.1/DTD/svg11.dtd">
"#;

    let full_svg = format!("{}{}", header, svg_content);
    let svg_data = full_svg.into_bytes();
    render_svg_to_png(svg_data).await
}

// 共同的SVG渲染逻辑
async fn render_svg_to_png(svg_data: Vec<u8>) -> Result<Response> {
    let opt = usvg::Options::default();
    
    let rtree = usvg::Tree::from_data(&svg_data, &opt.to_ref())
        .map_err(|err| format!("failed to decode SVG: {}", err))?;
    
    let pixmap_size = rtree.svg_node().size.to_screen_size();
    let mut pixmap = tiny_skia::Pixmap::new(pixmap_size.width(), pixmap_size.height())
        .ok_or("failed to create new pixmap")?;
    
    resvg::render(
        &rtree,
        usvg::FitTo::Original,
        tiny_skia::Transform::default(),
        pixmap.as_mut(),
    )
    .ok_or("failed to render PNG")?;
    
    let out = pixmap
        .encode_png()
        .map_err(|err| format!("failed to encode PNG: {}", err))?;
    
    let mut headers = Headers::new();
    headers.set("content-type", "image/png").unwrap();
    headers.set("cache-control", "public, max-age=3600").unwrap(); // 添加缓存头
    
    Ok(Response::from_bytes(out).unwrap().with_headers(headers))
}
