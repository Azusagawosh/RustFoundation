//Переменные и изменяемость\\
/*
fn main() {
let x = 5;
println!("{}, x");
x = 6; // тут выходит ошибка так как попытались присвоить новое значение неизменяемой переменной х.
println!("{}, x");
}
*/ 
/*
fn main() {
    let mut x = 5; // тут мы применили mut таким образом мы можем теперь менять значение переменной
                   // так как она стала изменяемой.
    println!("{}, x");
    x = 6;
    println!("{}, x");
}
*/

//Константы\\
/*
Во-первых, нельзя использовать mut с константами. Константы не просто неизменяемы
по умолчанию — они неизменяемы всегда. Для объявления констант используется ключевое
слово const вместо let, а также тип значения должен быть указан в аннотации.
*/
/*
fn main() {
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
}
*/

//Затенение (переменных)\\
/*
fn main() {
    let x = 5;
    let x = x +1; 
    {
        let x = x * 5;
        println!("{}, x);
    }

    println!("{}, x");
}
*/
/*
Сначала x = 5. Потом let x создаёт новую x на основе старой: 5 + 1 = 6. Затем внутри
блока {} третья let x снова создаёт новую x: 6 * 2 = 12. Когда блок заканчивается, эта
внутренняя x исчезает, и внешняя снова равна 6.
*/
//Кортежи\\
//Кортеж является общим способом совместной группировки нескольких значений
//азличного типа в единый комбинированный тип. Кортежи имеют фиксированную длину:
//после объявления они не могут расти или уменьшаться в размере.

/*
fn main() {
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);
}
*/
/*
fn main() {
    let x: (i32, f64, u8) = (28, 2.67, 1);
    let twenty_eight = x.0;
    let two_point_sixty_seven = x.1;
    let one = x.2;
    println!("{}, {}, {}", twenty_eight, two_point_sixty_seven, one);
}
*/
//Массивы\\
//Другим способом получения коллекции из множества значений является массив.
//В отличии от кортежа, каждый элемент массива имеет одинаковый тип. Массивы в Rust
//отличаются от массивов в некоторых других языках тем, что в Rust они, подобно кортежам,
//имеют фиксированную длину.
//В Rust, значения, попадающие в массив, записываются как список разделённых запятой
//значений внутри квадратных скобок:
/*
fn main() {
    let a: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let first = a[0];
    let second = a[1];
    let five = a[4];
    println!("{}, {}, {}", first, second, five);

}
*/
/*
fn main() {
let x: (i32, char, bool) = (19, 'C', false);
let nine_teen = x.0;
let c = x.1;
let boolean = x.2;
println!("{}, {}, {}", nine_teen, c, boolean);
}
*/
/*
fn main() {
   let x: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
   let first = x[0];
   let second = x[1];
   let ten = x[9];
   let five = x[4];
   println!("{}, {}, {}, {}", first, second, ten, five); 
} 
*/

fn main() {
    let name = "azusagawosh";
    let ocenka: [i32; 3] = [90, 90, 100];
    let first = ocenka[0];
    let second = ocenka[1];
    let three = ocenka[2];
    let info: (&str, bool, i32) = ("четвертый курс", true, 19);
    let course = info.0;
    let status = info.1;
    let average = ocenka[0] + ocenka[1] + ocenka[2];
    let age = info.2;
    let grade = ocenka.len();
    let all = average as f64 / grade as f64;
    println!("имя студента: {}, возвраст студента: {}, статус студента: {}, курс студента: {}, Оценки: {}, {}, {}", name, age, status, course, first, second, three);
    println!("сумма azusagawosh: {}", average);
    println!("кол во оценок: {}", grade);
    println!("средний балл: {:.2}", all);
} 



