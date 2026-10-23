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
//Выражения if\\
//Выражение if позволяет выполнять части кода в зависимости от условий. Вы задаёте\\
//условие, а затем указываете: "Если это условие выполняется, выполните этот блок кода. Если\\
//условие не выполняется, не выполняйте этот блок кода".\\
/*
fn main() {
let number = 3;

if number < 5 {
println!("condition was true");
} else {
println!("confition was false");
}
}
*/
//Обработка нескольких условий с помощью else if\\
//Можно использовать несколько условий, комбинируя if и else в выражении else if.\\
//Например:\\
/*
fn main() {
    let number = 6;


    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {

        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    
    }
}
*/
//тут будет выполнено до 2 го принта и так как это совпадает программа дальше не будет проверять

//Использование if в инструкции let\\
/*
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

}
*/
//тут у нас должны быть одинаковые типы\\

/* training
fn main() {
    let number = 99;
    if number < 100 {
        println!("true");
    } else if number == 99 {
        println!("true");
    } else {
        println!("false");
    }
}
*/

/*
fn main() {
    let prices: [i32; 4] = [20000, 15000, 45000, 36000];
    let customer: (&str, i32, bool) = ("azusagawosh", 19, true);
    let name = customer.0;
    let age = customer.1;
    let status = customer.2;
    let money = 130000;
    let total = prices[0] + prices[1] + prices[2] + prices[3];
    let length = prices.len();
    let average = total as f64 / length as f64;
    let max = prices[0].max(prices[1]).max(prices[2]).max(prices[3]);
    let min = prices[0].min(prices[1]).min(prices[2]).min(prices[3]);
    if age >= 18 {
        println!("покупка доступна");
    } else {
        println!("покупка недоступна");
    }

    if money >= total {
        println!("средства успешно списаны, благодарим за покупку");
    } else {
        println!("у вас недостаточно средств для покупки");
    }

    if age < 18 {
        println!("возраст не соответствует");
    } else if money < total {
        println!("возраст соответствует но сумма не достаточная");
    } else if status {
        println!("возраст соответствует и сумма достаточная, статус проведен");
    } else {
        println!("возраст соответствует сумма достаточная но статус еще не проведен");
    } 

    println!("имя: {}, возраст: {}, баланс: {}", name, age, money);
    println!("стоимость покупки: {}", total);
    println!("средняя цена: {:.2}", average);
    println!("максимальная сумма: {}, минимальная сумма: {}", max, min);
} 
*/
/*
fn main() {
    let prices: [i32; 5] = [1200, 3500, 800, 4200, 2300];
    let customer: (&str, i32, bool) = ("azusagawosh", 19, true);
    let name = customer.0;
    let age = customer.1;
    let status = customer.2;
    let money = 15000;
    let total = prices[0] + prices[1] + prices[2] + prices[3] +  prices[4];
    let length = prices.len();
    let average = total as f64 / length as f64;
    let max = prices[0].max(prices[1]).max(prices[2]).max(prices[3]).max(prices[4]);
    let min = prices[0].min(prices[1]).min(prices[2]).min(prices[3]).min(prices[4]);
    let discount: i32;
    if age < 18 {
        println!("покупка запрещена");
    } else if money < total {
        println!("денег не достаточно");
    } else if status {
        println!("заказ оплачен");
    } else {
        println!("заказ готов к оплате");
    }
    
    if total < 5000 {
        discount = 0;
    } else if total < 10000 { 
        discount = 5;
    } else if total < 15000 {
        discount = 10;
    } else {
        discount = 15;
    }
    println!("максимальная: {}, минимальная: {}", max, min);
    println!("средняя стоимость товаров: {:.2}", average);
    println!("общая сумма товаров: {}", total);
    println!("имя: {}, возраст: {}, баланс: {}", name, age, money);
    println!("Скидка в деньгах: {}", total * discount / 100);
    println!("Итоговая стоимость: {}", total - (total * discount / 100));
}
*/

//Повторное выполнение кода с помощью циклов\\
//В Rust есть три вида циклов: loop, while и for. Давайте попробуем каждый из них.
//Повторение выполнения кода с помощью loop
//Ключевое слово loop говорит Rust выполнять блок кода снова и снова до бесконечности
//пока не будет явно приказано остановиться.

/*
fn main() {
    loop {
        println!("again!");
    }
}
*/
//Когда запустим эту программу, увидим, как again! печатается снова и снова, пока не остановить программу вручную.
//К счастью, Rust также предоставляет способ выйти из цикла с помощью кода. 
//Ключевое слово break нужно поместить в цикл, чтобы указать программе, когда следует прекратить
//выполнение цикла.
//
//Возвращение значений из циклов
//Одно из применений loop - это повторение операции, которая может закончиться неудачей,
//например, проверка успешности выполнения потоком своего задания.
/*
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}
*/
/*
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}
*/
/*
fn main() {
    let mut sum = 69;
    'up: loop {
        println!("sum = {sum}");
        let mut num = 67;

        loop {
            println!("num = {num}");
            if num == 60 {
                break;
            }
            if sum == 60 {
                break 'up;
            }
            num -= 1; 
        }
        sum -= 3;
    }
    println!("end sum = {sum}");
}
*/
/*
fn main() {
    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }
}
*/

//Это возможно, чтобы break или continue наружные петли при работе с вложенным Петли.
//В этих случаях петли должны быть аннотированы с некоторыми 'label,
//и этикетка должна быть передана в break// continue Заявление.

/*
#![allow(unreachable_code, unused_labels)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    println!("Exited the outer loop");
}
*/

//Одно из применений loopПовторно пытаться оперировать до тех пор, пока она не преуспеет. 
//Если то операция возвращает значение, хотя вам может понадобиться передать его
//остальной части Код: положить его после break, и он будет возвращен loop Выражение.
/*
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("{}", result);
    assert_eq!(result, 20);
}
*/
/*
fn main() {
    let mut num = 0;
    let result = loop {
        num += 10;
        if num == 100 {
            println!("one hundreed");
            
            continue;
        }
        
        if num == 200 {
        break num * 16;
        }
        
    };
    println!("{}", result);
}
*/
/*
fn main() {
   let mut num = 0;
   let result = loop {
       num += 10;
       if num == 100 {
           println!("one_hundreed");
           continue;
       }
       if num == 500 {
           break num * 3;
       }
   };
   println!("{}", result);
}
*/

//Циклы с условием while\\
//В программе часто требуется проверить состояние условия в цикле. Пока условие истинно,
//цикл выполняется. Когда условие перестаёт быть истинным, программа вызывает break,
//останавливая цикл. Такое поведение можно реализовать с помощью комбинации loop, if,
//else и break. При желании попробуйте сделать это в программе. Это настолько
//распространённый паттерн, что в Rust реализована встроенная языковая конструкция для него, называемая цикл while.
/*
fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
*/
/*
fn main() {
    let num: [i32; 5] = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("{}", num[index]);
        index += 1;
    }
}
*/
/*
fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}
*/
/*
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
*/
/*
fn main() {
    let num: [i32; 4] = [100, 1200, 500, 300];
    let num1 = 450;
    let length = num.len();
    let total = num[0] + num[1] + num[2] + num[3];
    let average = total as f64 / length as f64;
    let info: (&str, &str, i32, i32) = ("Eren Yeader", "Levi Akkerman", 19, 23);
    let antagonist = info.0;
    let hero = info.1;
    let age1 = info.2;
    let age2 = info.3;
    let total1 = total + num1;
    let awards: [i32; 4] = [100, 500, 1000, 200];
    for element in awards {
        println!("заслуги Леви Аккермана, {element} золотых монет");
    }
    let mut choice = 0;
    'up: loop {
        choice += 1;
        if choice == 1 {
            println!("Стены в порядке, титанов не обнаружено.");
        } else if choice == 2 {
            println!("Разведкорпус отправляется за стены!");
        } else if choice == 3 {
            println!("завершение работы.");
            break 'up;
        } else {
            println!("error");
        }
    }
    println!("имя нападающего: {}, возраст: {}", antagonist, age1);
    println!("имя героя: {}, возраст: {}", hero, age2);
    println!("общее количество воевавщих: {}", total1);
    println!("общее кол во умерщих в бою с Ереном: {}", total);
    println!("общее кол во спасенных Леви Аккерманом: {}", num1);
    println!("среднее количество мертвых солдатов: {:.2}", average);
}
*/
//Управляющая конструкция match\\
//В Rust есть чрезвычайно мощный механизм управления потоком, именуемый match,
//который позволяет сравнивать значение с различными шаблонами и затем выполнять код в
//зависимости от того, какой из шаблонов совпал. Шаблоны могут состоять из литеральных
//значений, имён переменных, подстановочных знаков и многого другого;
//Думайте о выражении match как о машине для сортировки монет: монеты скользят по дорожке с различными по размеру отверстиями,
//и каждая монета падает через первое попавшееся отверстие, в которое она поместилась.
//аким же образом значения проходят через каждый шаблон в match,
//и при первом же "подходящем" шаблоне значение попадает в соответствующий блок кода, который будет использоваться во время выполнения.
/*
fn main() {
    let mut choice = 0;
    'up: loop {
        choice += 1;

        match choice {
            1 => println!("one: {choice}"),
            2 => println!("two: {choice}"),
            3 => {
                println!("three: {choice}");
                break 'up;
            }
            _ => println!("Error"), // Ловит все остальные числа
        }
    }
}
*/
/*
fn main() {
    let mut num = 0;
    'upp: loop {
        num += 10;

        match num {
            10 => println!("ten: {num}"),
            100 => println!("one hundreed: {num}"),
            1000 => println!("one thausand: {num}"),
            10000 => {
                println!("ten thausand: {num}");
                break 'upp;
            }
            _ => println!("Error"),
        }
    }
}
*/
/*
 fn main() {
    let num: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for element in num {
        println!("numbers: {element}");
    } 
}

fn main() {
    for num in 1..11 {
        println!("numbers: {num}");
    }
}
*/
/*
fn main() {
'outer: for i in 1..=3 {
    for j in 1..=3 {
        if i == j {
            continue;
        }
        println!("{i}, {j}");
    }
}
}
*/

fn main() {
    let num: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let mut index = 0;
    while index < 10 {
        let current = num[index];
        index += 1;
        if current % 2 == 0 {
            continue;
        } 
            println!("{}", current);
         
    }
}
