using Microsoft.AspNetCore.Mvc;

namespace backend.Controllers;

public class ChessController : ControllerBase
{
    [HttpGet("/")]
    public string Test()
    {
        return "Wasgood";
    }
}