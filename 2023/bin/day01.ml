let is_digit = function
  | '0' .. '9' -> true
  | _ -> false
;;

let only_digits str =
  String.fold_left
    (fun acc c -> if is_digit c then acc ^ String.make 1 c else acc)
    ""
    str
;;

let remove_middle_digits s =
  let length = String.length s in
  String.make 1 (String.get s 0) ^ String.make 1 (String.get s (length - 1))
;;

let rec part1 = function
  | [] -> 0
  | h :: t -> (only_digits h |> remove_middle_digits |> int_of_string) + part1 t
;;

let convert_digit_words_to_numbers s =
  let open Str in
  s
  |> global_replace (regexp "one") "one1one"
  |> global_replace (regexp "two") "two2two"
  |> global_replace (regexp "three") "three3three"
  |> global_replace (regexp "four") "four4four"
  |> global_replace (regexp "five") "five5five"
  |> global_replace (regexp "six") "six6six"
  |> global_replace (regexp "seven") "seven7seven"
  |> global_replace (regexp "eight") "eight8eight"
  |> global_replace (regexp "nine") "nine9nine"
;;

let rec part2 = function
  | [] -> 0
  | h :: t ->
    (convert_digit_words_to_numbers h
     |> only_digits
     |> remove_middle_digits
     |> int_of_string)
    + part2 t
;;

let () =
  let lines = Aoc.read_lines "./input/day01.prod.input" in
  Printf.printf "part1: %d\n" (part1 lines);
  Printf.printf "part2: %d\n" (part2 lines)
;;
