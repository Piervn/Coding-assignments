let arrayDiff itemsToExclude source = 
    source |> Array.filter (fun x -> not <| Array.contains x itemsToExclude)

let itemsToExclude = [|1; 2; 3|]
let source = [|1; 2; 3; 4; 5|]
let result = arrayDiff itemsToExclude source
printfn "%A" result
