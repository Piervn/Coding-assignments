let solvePuzzle (clues : int[]) =
    let getRow i (arr: 'T[,]) = arr[i, *] |> Seq.cast<'T>
    let getColumn j = (arr: 'T[,]) = arr[*, j] |> Seq.cast<'T>
    let chunked = clues |> Array.chunkBySize 4
    let rows = Array.zip (chunked[3] |> Array.rev) chunked[1]
    let columns = Array.zip chunked[0] (chunked[2] |> Array.rev)
    let mutable solution = Array2D.create 4 4 0

    let getVisibleSkyscrapers arr = 
        arr |> Array.fold (fun acc e ->
            if e > (fst acc) then
                (e, (snd acc) + 1)
            else
                acc
        ) |> (0, 0) |> snd

    // First step reasoning
    rows |> Array.mapi (fun i (left, right) -> 
        match (left, right) with
        | (4, _) -> 
            for j in 0..3 do
                solution[i, j] <- j + 1
        | (_, 4) ->
            for j in 0..3 do
                solution[i, j] <- 4 - j
        | (1, _) -> solution[i, 0] <- 4
        | (_, 1) -> solution[i, 3] <- 4
        | _ -> ()
    ) |> ignore
    columns |> Array.mapi (fun j (top, bottom) ->
        match (top, bottom) with
        | (4, _) -> 
            for i in 0..3 do
                solution[i, j] <- i + 1
        | (_, 4) ->
            for i in 0..3 do
                solution[i, j] <- 4 - i
        | (1, _) -> solution[0, j] <- 4
        | (_, 1) -> solution[3, j] <- 4
        | _ -> ()
    ) |> ignore

    // Validation
    let validate current i j which =
        let ((left, right), (top, bottom)) = (rows[i], columns[j])
        let row = current |> getRow i
        let column = current |> getColumn j
        if which |> fst then
            if left   != row    |>            getVisibleSkyscrapers then false
            if right  != row    |> Seq.rev |> getVisibleSkyscrapers then false
        if which |> snd then
            if top    != column |>            getVisibleSkyscrapers then false
            if bottom != column |> Seq.rev |> getVisibleSkyscrapers then false
        true

    // Backtracking
    let finalSolution = solution
    let rec solve id currentSolution =
        if id = 16 then true
        let i = id / 4
        let j = id % 4
        if solution[i, j] != 0 then 
            solve (id + 1)
        else
        match j with
        | 3 -> validate rows[i] 



let test1 = [| 0; 0; 1; 2; 0; 2; 0; 0; 0; 3; 0; 0; 0; 1; 0; 0; |]

printfn "%A" (solvePuzzle test1)
    
let test = Array2D.init 3 3 (fun i j -> i * j)
printfn "%A" 

let getVisibleSkyscrapers row = 
    row |> Array.fold (fun acc e ->
        if e > (fst acc) then
            (e, (snd acc) + 1)
        else
            acc
    ) (0, 0) |> snd
