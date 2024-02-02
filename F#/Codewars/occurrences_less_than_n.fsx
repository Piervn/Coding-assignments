let deleteNth order max_e =
  order |> List.fold (fun acc e -> 
    let count = acc |> List.filter ((=) e) |> List.length
    if count < max_e then e::acc else acc) 
    [] 
    |> List.rev

let res1 = deleteNth [20; 37; 20; 21] 1 
let res2 = deleteNth [1; 1; 3; 3; 7; 2; 2; 2; 2] 3
res1 |> printfn "%A"
res2 |> printfn "%A"

[20; 37; 21] = res1 |> printfn "%A"
[1; 1; 3; 3; 7; 2; 2; 2] = res2 |> printfn "%A" 