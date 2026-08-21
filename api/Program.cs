const string applicationName = "[ZXG] Réparateur Mod Tools BO3";
const string version = "1.0.0";
const string releaseDate = "2026-08-21";
const string downloadUrl =
    "https://github.com/starzismik/ZXG-Reparateur-Mod-Tools-BO3/releases/latest";

var builder = WebApplication.CreateBuilder(args);
var app = builder.Build();

app.MapGet("/", () => Results.Ok(new
{
    application = applicationName,
    status = "online"
}));

app.MapGet("/health", () => Results.Ok(new
{
    application = applicationName,
    status = "healthy"
}));

app.MapGet("/update", () => Results.Ok(new
{
    application = applicationName,
    version,
    release_date = releaseDate,
    download_url = downloadUrl
}));

app.Run();
