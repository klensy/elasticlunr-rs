use crate::Pipeline;

pub fn make_pipeline() -> Pipeline {
    Pipeline {
        queue: vec![
            ("trimmer-ru".into(), trimmer),
            ("stopWordFilter-ru".into(), stop_word_filter),
            ("stemmer-ru".into(), stemmer),
        ],
    }
}

make_trimmer!(
    "\\u0400-\\u0484\\u0487-\\u052F\\u1D2B\\u1D78\\u2DE0-\\u2DFF\\uA640-\\uA69F\\uFE2E\\uFE2F"
);

make_stop_word_filter!([
    "",
    "алло",
    "без",
    "близко",
    "более",
    "больше",
    "будем",
    "будет",
    "будете",
    "будешь",
    "будто",
    "буду",
    "будут",
    "будь",
    "бы",
    "бывает",
    "бывь",
    "был",
    "была",
    "были",
    "было",
    "быть",
    "в",
    "важная",
    "важное",
    "важные",
    "важный",
    "вам",
    "вами",
    "вас",
    "ваш",
    "ваша",
    "ваше",
    "ваши",
    "вверх",
    "вдали",
    "вдруг",
    "ведь",
    "везде",
    "весь",
    "вниз",
    "внизу",
    "во",
    "вокруг",
    "вон",
    "восемнадцатый",
    "восемнадцать",
    "восемь",
    "восьмой",
    "вот",
    "впрочем",
    "времени",
    "время",
    "все",
    "всегда",
    "всего",
    "всем",
    "всеми",
    "всему",
    "всех",
    "всею",
    "всю",
    "всюду",
    "вся",
    "всё",
    "второй",
    "вы",
    "г",
    "где",
    "говорил",
    "говорит",
    "год",
    "года",
    "году",
    "да",
    "давно",
    "даже",
    "далеко",
    "дальше",
    "даром",
    "два",
    "двадцатый",
    "двадцать",
    "две",
    "двенадцатый",
    "двенадцать",
    "двух",
    "девятнадцатый",
    "девятнадцать",
    "девятый",
    "девять",
    "действительно",
    "дел",
    "день",
    "десятый",
    "десять",
    "для",
    "до",
    "довольно",
    "долго",
    "должно",
    "другая",
    "другие",
    "других",
    "друго",
    "другое",
    "другой",
    "е",
    "его",
    "ее",
    "ей",
    "ему",
    "если",
    "есть",
    "еще",
    "ещё",
    "ею",
    "её",
    "ж",
    "же",
    "жизнь",
    "за",
    "занят",
    "занята",
    "занято",
    "заняты",
    "затем",
    "зато",
    "зачем",
    "здесь",
    "значит",
    "и",
    "из",
    "или",
    "им",
    "именно",
    "иметь",
    "ими",
    "имя",
    "иногда",
    "их",
    "к",
    "каждая",
    "каждое",
    "каждые",
    "каждый",
    "кажется",
    "как",
    "какая",
    "какой",
    "кем",
    "когда",
    "кого",
    "ком",
    "кому",
    "конечно",
    "которая",
    "которого",
    "которой",
    "которые",
    "который",
    "которых",
    "кроме",
    "кругом",
    "кто",
    "куда",
    "лет",
    "ли",
    "лишь",
    "лучше",
    "люди",
    "м",
    "мало",
    "между",
    "меля",
    "менее",
    "меньше",
    "меня",
    "миллионов",
    "мимо",
    "мира",
    "мне",
    "много",
    "многочисленная",
    "многочисленное",
    "многочисленные",
    "многочисленный",
    "мной",
    "мною",
    "мог",
    "могут",
    "мож",
    "может",
    "можно",
    "можхо",
    "мои",
    "мой",
    "мор",
    "мочь",
    "моя",
    "моё",
    "мы",
    "на",
    "наверху",
    "над",
    "надо",
    "назад",
    "наиболее",
    "наконец",
    "нам",
    "нами",
    "нас",
    "начала",
    "наш",
    "наша",
    "наше",
    "наши",
    "не",
    "него",
    "недавно",
    "недалеко",
    "нее",
    "ней",
    "нельзя",
    "нем",
    "немного",
    "нему",
    "непрерывно",
    "нередко",
    "несколько",
    "нет",
    "нею",
    "неё",
    "ни",
    "нибудь",
    "ниже",
    "низко",
    "никогда",
    "никуда",
    "ними",
    "них",
    "ничего",
    "но",
    "ну",
    "нужно",
    "нх",
    "о",
    "об",
    "оба",
    "обычно",
    "один",
    "одиннадцатый",
    "одиннадцать",
    "однажды",
    "однако",
    "одного",
    "одной",
    "около",
    "он",
    "она",
    "они",
    "оно",
    "опять",
    "особенно",
    "от",
    "отовсюду",
    "отсюда",
    "очень",
    "первый",
    "перед",
    "по",
    "под",
    "пожалуйста",
    "позже",
    "пока",
    "пор",
    "пора",
    "после",
    "посреди",
    "потом",
    "потому",
    "почему",
    "почти",
    "прекрасно",
    "при",
    "про",
    "просто",
    "против",
    "процентов",
    "пятнадцатый",
    "пятнадцать",
    "пятый",
    "пять",
    "раз",
    "разве",
    "рано",
    "раньше",
    "рядом",
    "с",
    "сам",
    "сама",
    "сами",
    "самим",
    "самими",
    "самих",
    "само",
    "самого",
    "самой",
    "самом",
    "самому",
    "саму",
    "свое",
    "своего",
    "своей",
    "свои",
    "своих",
    "свою",
    "сеаой",
    "себе",
    "себя",
    "сегодня",
    "седьмой",
    "сейчас",
    "семнадцатый",
    "семнадцать",
    "семь",
    "сих",
    "сказал",
    "сказала",
    "сказать",
    "сколько",
    "слишком",
    "сначала",
    "снова",
    "со",
    "собой",
    "собою",
    "совсем",
    "спасибо",
    "стал",
    "суть",
    "т",
    "та",
    "так",
    "такая",
    "также",
    "такие",
    "такое",
    "такой",
    "там",
    "твой",
    "твоя",
    "твоё",
    "те",
    "тебе",
    "тебя",
    "тем",
    "теми",
    "теперь",
    "тех",
    "то",
    "тобой",
    "тобою",
    "тогда",
    "того",
    "тоже",
    "только",
    "том",
    "тому",
    "тот",
    "тою",
    "третий",
    "три",
    "тринадцатый",
    "тринадцать",
    "ту",
    "туда",
    "тут",
    "ты",
    "тысяч",
    "у",
    "уж",
    "уже",
    "уметь",
    "хорошо",
    "хотеть",
    "хоть",
    "хотя",
    "хочешь",
    "часто",
    "чаще",
    "чего",
    "человек",
    "чем",
    "чему",
    "через",
    "четвертый",
    "четыре",
    "четырнадцатый",
    "четырнадцать",
    "что",
    "чтоб",
    "чтобы",
    "чуть",
    "шестнадцатый",
    "шестнадцать",
    "шестой",
    "шесть",
    "эта",
    "эти",
    "этим",
    "этими",
    "этих",
    "это",
    "этого",
    "этой",
    "этом",
    "этому",
    "этот",
    "эту",
    "я",
    "﻿а",
]);

make_stemmer!(Algorithm::Russian);
