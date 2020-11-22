use handlebars::{Context, Handlebars, Helper, Output, RenderContext, RenderError};

fn app_name_helper(
    _: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext,
    out: &mut dyn Output,
) -> Result<(), RenderError> {
    let app_name = dotenv::var("APP_NAME").unwrap_or_else(|_| "MiniPress".to_string());
    out.write(app_name.as_ref())?;
    Ok(())
}

pub fn register_helpers(hb: &mut Handlebars) {
    hb.register_helper("app_name", Box::new(app_name_helper));
}
