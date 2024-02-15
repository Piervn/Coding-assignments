let rgb r g b =
    let clamp = max 0 >> min 255
    sprintf "%02X%02X%02X" (r |> clamp) (g |> clamp) (b |> clamp)

printfn "%A" ("FFFFFF" = rgb 255 255 255)
printfn "%A" ("FFFFFF" = rgb 255 255 300)
printfn "%A" ("000000" = rgb 0 0 0)
printfn "%A" ("9400D3" = rgb 148 0 211)
printfn "%A" ("9400D3" = rgb 148 -20 211)
printfn "%A" ("90C3D4" = rgb 144 195 212)
printfn "%A" ("D4350C" = rgb 212 53 12)