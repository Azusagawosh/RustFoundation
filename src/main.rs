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
/*
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
*/
/*
fn main() {
let name = "azusagawosh";
let age = 19;
let stat = true;
let product: (&str, &str, &str) = ("mouse", "keyboard", "headphones");
let m = product.0;
let k = product.1;
let h = product.2;
let prices: [i32; 3] = [12000, 23000, 17000];
let total = prices[0] + prices[1] + prices[2];
let length = prices.len();
let average = total as f64 / length as f64;
println!("name is {}, age {}, status {},", name, age, stat);
println!("покупает {}, {}, {}", m, k, h);
println!("колво продуктов который берет покупатель {}", length);
println!("общая стоимость: {} тенге",total);
println!("среднее значение: {:.2}", average);
}*/
/*
fn main() {
let prices: [i32; 5] = [12000, 10000, 20000, 7000, 17000];
let product: (&str, &str, &str, &str, &str) = ("iphone", "ThinkPad", "headphones", "mouse", "keyboard");
let one = product.0;
let two = product.1;
let three = product.2;
let four = product.3;
let five = product.4;
let total = prices[0] + prices[1] + prices[2] + prices[3] + prices[4];
let length = prices.len();
let average = total as f64 / length as f64;
let payment = 100000;
let change = payment - total;п
println!("продукты которые взял что бы купить: {}, {}, {}, {}, {}", one, two, three, four, five);
println!("кол во продуктов: {}", length);
println!("общая сумма товаров: {}", total);
println!("сумма которую дал покупатель за товары: {}", payment);
println!("сумма сдачи которую должен вернуть кассир: {}", change);
println!("среднее количество от суммы: {:.2}", average);
}
*/
/*
fn main() {
let prices: [i32; 3] = [10000, 20000, 30000];
let product: (&str, &str, &str) = ("headphones", "iphone", "ThinkPad");
let one = product.0;
let two = product.1;
let three = product.2;
let total = prices[0] + prices[1] + prices[2];
let length = prices.len();
let average = total as f64 / length as f64;
let payment = 80000;
let change = payment - total;
let sum: [i32; 2] = [2500, 2500];
let tot = sum[0] + sum[1];
let len = sum.len();
let aver = tot as f64 / len as f64;
let paying = 5000;
let changing = change - tot;
let biggest = prices[0].max(prices[1]).max(prices[2]);
let minimal = prices[0].min(prices[1]).min(prices[2]);
let all_total = total + tot;
println!("продукты которые покупает: {}, {}, {}", one, two, three);
println!("сумма которую оплатил: {}", payment);
println!("сдача которую получит: {}", change);
println!("что то купил после первой покупки: {}", paying);
println!("остаток который остался после повторной покупки: {}", changing);
println!("самая высокая цена: {}", biggest);
println!("самая низкая цена: {}", minimal);
println!("общая сумма покупок: {}", all_total);
println!("название товара у которого самая высокая цена: {}, цена: {}", three, biggest);
}
*/
/*
fn main() {
let prices: [i32; 5] = [80000, 100000, 10000, 70000, 30000];
let product: (&str, &str, &str, &str, &str) = ("iphone", "ThinkPad", "mouse", "android", "headphones");
let one = product.0;
let two = product.1;
let three = product.2;
let four = product.3;
let five = product.4;
let total = prices[0] + prices[1] + prices[2] + prices[3] + prices[4];
let max = prices[0].max(prices[1]).max(prices[2]).max(prices[3]).max(prices[4]);
let min = prices[0].min(prices[1]).min(prices[2]).min(prices[3]).min(prices[4]);
let length = prices.len();
let average = total as f64 / length as f64;
println!("самая большая цена товаров: {}", max);
println!("самая маленькая цена товаров: {}", min);
println!("общая цена товаров: {}", total);
println!("средняя цена: {}", average);
println!("название товара у которой большая цена: {}", two);
} */
/*
fn main() {
let prices: [i32; 4] = [1500, 2500, 1000, 5000];
let purchase: (&str, i32, bool) = ("azusagawosh", 19, true);
let name = purchase.0;
let age = purchase.1;
let status = purchase.2;
let total = prices[0] + prices[1] + prices[2] + prices[3];
let length = prices.len();
let average = total as f64 / length as f64;
let min = prices[0].min(prices[1]).min(prices[2]).min(prices[3]);
let max = prices[0].max(prices[1]).max(prices[2]).max(prices[3]);
println!("данные клиента - имя: {}, возраст: {}, статус оплаты: {}, сумма оплаты: {}", name, age, status, total);
println!("средняя цена: {:.2}", average);
println!("самая большая сумма товаров: {}", max);
println!("самая маленькая сумма товаров: {}", min);
}
*/
