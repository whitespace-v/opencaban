use super::structs::{
    CarSecList, CarSecListDescription, CarSecListDescriptionRepair, CarSecListExtendedBrakes,
    CarSecListExtendedElectrics, CarSecListExtendedFuel, CarSecListExtendedSteering,
    CarSecListExtendedSteeringCondition, CarSecListExtendedTable, CarSecListExtendedTableChassis,
    CarSecListExtendedTableDiagnostics, CarSecListExtendedTableEngine,
    CarSecListExtendedTableEngineCoolantLeak, CarSecListExtendedTableEngineOilLeak,
    CarSecListExtendedTableTransmission, CarSecListExtendedTableTransmissionAutomatic,
    CarSecListExtendedTableTransmissionManual, CarSecListExtendedWiring, CarSecListOpinions,
};
use crate::{
    extractor::extract::{
        extract_attr, extract_attrs, extract_value, extract_values, with_checked,
    },
    kbchachacha::pagination::seclist::structs::CarSecListPointSchemeItem,
};
use scraper::Html;
use std::error::Error;

pub fn parse(document: &Html) -> Result<CarSecList, Box<dyn Error>> {
    let seclist_num = extract_value(document, "div.docu_title");
    // 1. название; 2. ГРЗ; 3. год выпуска; 4. срок действия тех.отчета;
    // 5. дата первой регистрации 7.идентификационный номер авто; 10. мотор
    let table1 = extract_values(document, "table.ins_tbl1 > tbody > tr > td");
    // 7. КПП
    let transmission_types = with_checked(
        document,
        "table.ins_tbl1 > tbody > tr:nth-child(3) > td > ul.chkbox_list > li",
    );
    // топливо
    let fuel_type = with_checked(
        document,
        "table.ins_tbl1 > tbody > tr:nth-child(5) > td > ul.chkbox_list > li",
    );
    // тип гарантии
    let warranty_type = with_checked(
        document,
        "table.ins_tbl1 > tbody > tr:nth-child(6) > td > ul.chkbox_list > li",
    );
    // состояние Пробега
    let odometr_status = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(3) > td > ul.chkbox_list > li",
    );
    // общее состояние авто
    let mileage_status = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(4) > td > ul.chkbox_list > li ",
    );
    // пробег в км
    let mileage_value = extract_value(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(2) > :nth-child(3)",
    );
    // состояние шильдика с вином
    let vin_plate_status = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(4) > td > ul.chkbox_list > li",
    );
    // выбросы
    let emission_names = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(5) > td > ul.chkbox_list > li",
    );
    // показатели выбросов
    let emission_values = extract_values(document, "td.exhaust_gas");

    // тюнинг/модификации
    let tuning_status = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(6) > :nth-child(2) > ul.chkbox_list > li",
    );
    // законность тюнинга если есть
    let tuning_legality = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(6) > :nth-child(3) > ul.chkbox_list > li",
    );
    let tuning_type = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(6) > :nth-child(4) > ul.chkbox_list > li",
    );
    // особая история
    let incidents = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(7) > :nth-child(2) > ul.chkbox_list > li",
    );
    // наводнение/огонь
    let incidents_flood_fire = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(7) > :nth-child(3) > ul.chkbox_list > li",
    );
    // Изменение способа использования ??
    let ownership_changes_status = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(8) > :nth-child(2) > ul.chkbox_list > li",
    );
    // аренда/ продажа
    let ownership_changes_value = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(8) > :nth-child(3) > ul.chkbox_list > li",
    );
    // Цвет (а)/хроматический
    let color_changes_chrome = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(9) > :nth-child(2) > ul.chkbox_list > li",
    );
    // полная покраска // изменение цвета
    let color_changes_type = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(9) > :nth-child(3) > ul.chkbox_list > li",
    );
    // Основные опции
    let options_status = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(10) > :nth-child(2) > ul.chkbox_list > li",
    );
    // люк на крыше/навигация/ другое
    let options_list = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(10) > :nth-child(3) > ul.chkbox_list > li",
    );
    // подлежит отзыву: неприменимо/ применимо
    let feedback_status = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(11) > :nth-child(2) > ul.chkbox_list > li",
    );
    // реализация отзыва: было/ не было
    let feedback_value = with_checked(
        document,
        "table.ins_tbl2 > tbody > tr:nth-child(12) > :nth-child(4) > ul.chkbox_list > li",
    );
    //image
    let table3_1 = extract_attr(
        document,
        "src",
        "table.ins_tbl3 > tbody > :nth-child(2) > td > div > p > img",
    );
    // // история несчатный случаев
    // let table3_2 = with_checked(
    //     document,
    //     "table.ins_tbl3 > tbody > tr:nth-child(3) > :nth-child(2) > ul.chkbox_list > li",
    // );
    // // простой ремонт
    // let table3_2_1 = with_checked(
    //     document,
    //     "table.ins_tbl3 > tbody > tr:nth-child(3) > :nth-child(4) > ul.chkbox_list > li",
    // );
    // внешняя часть
    //// 1 ранг
    let table3_3 = with_checked(
        document,
        "table.ins_tbl3 > tbody > tr:nth-child(5) > td > ul.chkbox_list > li",
    );
    //// 2 ранг
    let table3_3_1 = with_checked(
        document,
        "table.ins_tbl3 > tbody > tr:nth-child(6) > td > ul.chkbox_list > li",
    );
    // основной скелет
    //// A ранг
    let table3_3_2 = with_checked(
        document,
        "table.ins_tbl3 > tbody > tr:nth-child(7) > td > ul.chkbox_list > li",
    );
    //// B ранг
    let table3_3_3 = with_checked(
        document,
        "table.ins_tbl3 > tbody > tr:nth-child(8) > td > ul.chkbox_list > li",
    );
    //// C ранг
    let table3_3_4 = with_checked(
        document,
        "table.ins_tbl3 > tbody > tr:nth-child(9) > td > ul.chkbox_list > li",
    );
    let mut scheme_unconverted: Vec<CarSecListPointSchemeItem> = vec![];
    scheme_unconverted.extend(table3_3);
    let ssss = extract_values(document, "ul.performance > li > strong");
    println!("{ssss:?}");
    let ssss = extract_attrs(document, "class", "ul.performance > li");
    println!("{ssss:?}");
    println!(
                "\n[Table3]\nimg: {table3_1}\ntable3_3: {table3_3:?}\ntable3_3_1: {table3_3_1:?}\ntable3_3_2: {table3_3_2:?}\ntable3_3_3: {table3_3_3:?}\ntable3_3_4: {table3_3_4:?}"
            );
    //table 4
    // самодиагностика
    //// мотор
    let diagnostics_engine = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(2) > td > ul.chkbox_list > li",
    )[0];
    //// кпп
    let diagnostics_transmission = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(3) > td > ul.chkbox_list > li",
    )[0];
    // мотор
    //// рабочее состояние (холостой ход)
    let engine_idling = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(4) > td > ul.chkbox_list > li",
    )[0];
    // масло
    //// Крышка цилиндра (крышка коромысла)
    let engine_oil_leak_valve_cover = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(5) > td > ul.chkbox_list > li",
    )[0];
    ///// Головка блока цилиндров / прокладка
    let engine_oil_leak_cylynder_head_gasket = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(6) > td > ul.chkbox_list > li",
    )[0];
    ///// Блок цилиндров / Масляный поддон Производитель Китай
    let engine_oil_leak_pan = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(7) > td > ul.chkbox_list > li",
    )[0];
    /////  Расход масла
    let engine_oil_pressure = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(8) > td > ul.chkbox_list > li",
    )[0];
    // Утечка охлаждающей жидкости
    ///// Головка блока цилиндров / прокладка
    let engine_coolant_leak_cylynder_head = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(9) > td > ul.chkbox_list > li",
    )[0];
    ///// водяной насос
    let engine_coolant_leak_pump = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(10) > td > ul.chkbox_list > li",
    )[0];
    ///// Радиатор
    let engine_coolant_leak_radiator = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(11) > td > ul.chkbox_list > li",
    )[0];
    ///// Количество охлаждающей жидкости
    let engine_coolant_leak_coolant_amount = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(12) > td > ul.chkbox_list > li",
    )[0];
    ///// Общая магистраль
    let engine_line = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(13) > td > ul.chkbox_list > li",
    )[0];
    // КПП
    ////// АКПП
    /// Утечка масла Масло
    let at_oil_leak = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(14) > td > ul.chkbox_list > li",
    )[0];
    /// Расход и состояние масла
    let at_oil_consumption = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(15) > td > ul.chkbox_list > li",
    )[0];
    /// Рабочее состояние (холостой ход)
    let at_idling = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(16) > td > ul.chkbox_list > li",
    )[0];
    ////// МКПП
    /// Утечка масла Масло
    let tm_oil_leak = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(17) > td > ul.chkbox_list > li",
    )[0];
    ///  Переключение передач
    let tm_gear_shift = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(18) > td > ul.chkbox_list > li",
    )[0];
    ///Расход и состояние масла
    let tm_oil_consumption = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(19) > td > ul.chkbox_list > li",
    )[0];
    /// Рабочее состояние (холостой ход)
    let tm_idling = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(20) > td > ul.chkbox_list > li",
    )[0];
    /////// Передача электроэнергии
    //Сцепление в сборе
    let clutch_assembly = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(21) > td > ul.chkbox_list > li",
    )[0];
    //Шарнир с постоянной скоростью вращения
    let joints = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(22) > td > ul.chkbox_list > li",
    )[0];
    //Приводной вал и подшипник
    let driveshaft = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(23) > td > ul.chkbox_list > li",
    )[0];
    //Дифференциальная передача
    let differential = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(24) > td > ul.chkbox_list > li",
    )[0];
    /////// Рулевое управление
    // Утечка масла при работе гидроусилителя рулевого управления
    let power_steering = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(25) > td > ul.chkbox_list > li",
    )[0];
    //// Рабочее состояние
    //Насос рулевого управления
    let steering_pump = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(26) > td > ul.chkbox_list > li",
    )[0];
    //Рулевой механизм с MDPS
    let steering_gear = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(27) > td > ul.chkbox_list > li",
    )[0];
    //Шарнир рулевого управления
    let steering_propshaft = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(28) > td > ul.chkbox_list > li",
    )[0];
    // Силовой шланг высокого давления
    let steering_hoses_n_tubes = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(29) > td > ul.chkbox_list > li",
    )[0];
    // Наконечник рулевой тяги и шаровой шарнир
    let steering_rack = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(30) > td > ul.chkbox_list > li",
    )[0];
    /////// тормозная система
    /// Утечка масла из Главного тормозного цилиндра
    let main_brake_cylinder_leak = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(31) > td > ul.chkbox_list > li",
    )[0];
    /// Утечка тормозного масла
    let brake_fluid_leak = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(32) > td > ul.chkbox_list > li",
    )[0];
    ///  Состояние источника питания
    let brake_fluid_sensor = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(33) > td > ul.chkbox_list > li",
    )[0];
    ////////// Электричество
    /// Выход генератора
    let electrics_generator = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(34) > td > ul.chkbox_list > li",
    )[0];
    /// Пусковой двигатель
    let electrics_starter = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(35) > td > ul.chkbox_list > li",
    )[0];
    /// Функция двигателя стеклоочистителя
    let electrics_wipers_electrics = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(36) > td > ul.chkbox_list > li",
    )[0];
    /// Двигатель для вентиляции помещений
    let electrics_engine_fan = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(37) > td > ul.chkbox_list > li",
    )[0];
    /// Двигатель вентилятора радиатора
    let electrics_engine_fan_motor = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(38) > td > ul.chkbox_list > li",
    )[0];
    /// Привод стеклоподъемника
    let electrics_window_lifter_drive = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(39) > td > ul.chkbox_list > li",
    )[0];
    ////////////////Классические источники Электрическое устройство
    // Состояние изоляции зарядного порта
    let charger_insulation = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(40) > td > ul.chkbox_list > li",
    )[0];
    // Состояние изоляции аккумуляторной батареи привода
    let battery_insulation = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(41) > td > ul.chkbox_list > li",
    )[0];
    //Состояние электропроводки высокой мощности (Соединительная клемма, ткань, защитный механизм)
    let high_power_wiring = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(42) > td > ul.chkbox_list > li",
    )[0];
    ///////////// Топливная система
    // Утечка топлива (включая сжиженный газ)
    let fuel_n_gas_leak = with_checked(
        document,
        "table.ins_tbl4 > tbody > tr:nth-child(43) > td > ul.chkbox_list > li",
    )[0];
    //// требуется ремонт
    // снаружи
    let appearance = with_checked(
        document,
        "table.ins_tbl5 > tbody > tr:nth-child(2) > :nth-child(3) > ul.chkbox_list > li",
    )[0];
    // внутри
    let interior = with_checked(
        document,
        "table.ins_tbl5 > tbody > tr:nth-child(2) > :nth-child(5) > ul.chkbox_list > li",
    )[0];
    // блеск
    let gloss = with_checked(
        document,
        "table.ins_tbl5 > tbody > tr:nth-child(3) > :nth-child(2) > ul.chkbox_list > li",
    )[0];
    // требуется Уборка
    let cleaning = with_checked(
        document,
        "table.ins_tbl5 > tbody > tr:nth-child(3) > :nth-child(4) > ul.chkbox_list > li",
    )[0];
    /////// Колеса
    let wheels_status = with_checked(
        document,
        "table.ins_tbl5 > tbody > tr:nth-child(4) > :nth-child(2) > ul.chkbox_list > li",
    )[0];
    // какое именно колесо либо критическая ситуация
    let wheels = with_checked(
        document,
        "table.ins_tbl5 > tbody > tr:nth-child(4) > :nth-child(3) > ul.chkbox_list > li",
    );
    ///////// шины
    let tires_status = with_checked(
        document,
        "table.ins_tbl5 > tbody > tr:nth-child(5) > :nth-child(2) > ul.chkbox_list > li",
    )[0];
    // какое именно колесо либо критическая ситуация
    let tires = with_checked(
        document,
        "table.ins_tbl5 > tbody > tr:nth-child(5) > :nth-child(3) > ul.chkbox_list > li",
    );
    // стекло
    let window = with_checked(
        document,
        "table.ins_tbl5 > tbody > tr:nth-child(6) > td > ul.chkbox_list > li",
    )[0];
    // основное
    let additional_items = with_checked(
                document,
                "table.ins_tbl5 > tbody > tr:nth-child(7) > :nth-child(3) > ul.chkbox_list:nth-child(1) > li",
            )[0];
    // ////// Особенности и мнения инспекторов
    // Производительность/ Комментарий Контролера
    let performance_n_health_inspector =
        extract_value(document, "table.ins_tbl5 > tbody > tr:nth-child(9) > td");
    //Цена/ Опрос Калькулятор
    let price_survey = extract_value(document, "table.ins_tbl5 > tbody > tr:nth-child(10) > td");
    // фотографии
    let images = extract_attrs(
        document,
        "src",
        "table.ins_tbl7 > tbody > tr > td.car_photo > ul > li > img",
    )?;
    // дата проведения техосмотра
    let table8_1 = extract_values(document, "table.ins_tbl8 > tbody > tr > td p.date");

    Ok(CarSecList {
        seclist_num,
        name: table1[0],
        ext_name: table1[0],
        license_plate: table1[1],
        release_year: table1[2],
        validity_period: table1[3],
        first_reg_date: table1[4],
        chassis_number: table1[6],
        transmission_types,
        fuel_type,
        engine: table1[9],
        warranty_type: warranty_type[0],
        odometr_status: odometr_status[0],
        mileage_status: mileage_status[0],
        mileage_value: mileage_value,
        vin_plate_status: vin_plate_status[0],
        emission_names,
        emission_values,
        tuning_status: tuning_status[0],
        tuning_legality: tuning_legality[0],
        tuning_type: tuning_type[0],
        incidents: incidents[0],
        incidents_flood_fire: incidents_flood_fire[0],
        ownership_changes_status: ownership_changes_status[0],
        ownership_changes_value: ownership_changes_value[0],
        color_changes_chrome: color_changes_chrome[0],
        color_changes_type: color_changes_type[0],
        options_status: options_status[0],
        options_list,
        feedback_status: feedback_status[0],
        feedback_value: feedback_value[0],
        // point_scheme: convert(scheme_unconverted),
        extended_table: CarSecListExtendedTable {
            diagnostics: CarSecListExtendedTableDiagnostics {
                diagnostics_engine,
                diagnostics_transmission,
            },
            engine: CarSecListExtendedTableEngine {
                engine_idling,
                engine_oil_leak: CarSecListExtendedTableEngineOilLeak {
                    engine_oil_leak_valve_cover,
                    engine_oil_leak_cylynder_head_gasket,
                    engine_oil_leak_pan,
                },
                engine_oil_pressure,
                engine_coolant_leak: CarSecListExtendedTableEngineCoolantLeak {
                    engine_coolant_leak_cylynder_head,
                    engine_coolant_leak_pump,
                    engine_coolant_leak_radiator,
                    engine_coolant_leak_coolant_amount,
                },
                engine_line,
            },
            transmission: CarSecListExtendedTableTransmission {
                automatic: CarSecListExtendedTableTransmissionAutomatic {
                    at_oil_leak,
                    at_oil_consumption,
                    at_idling,
                },
                manual: CarSecListExtendedTableTransmissionManual {
                    tm_oil_leak,
                    tm_gear_shift,
                    tm_oil_consumption,
                    tm_idling,
                },
            },
            chassis: CarSecListExtendedTableChassis {
                clutch_assembly,
                joints,
                driveshaft,
                differential,
            },
            steering: CarSecListExtendedSteering {
                power_steering,
                condition: CarSecListExtendedSteeringCondition {
                    steering_pump,
                    steering_gear,
                    steering_propshaft,
                    steering_hoses_n_tubes,
                    steering_rack,
                },
            },
            brakes: CarSecListExtendedBrakes {
                main_brake_cylinder_leak,
                brake_fluid_leak,
                brake_fluid_sensor,
            },
            electics: CarSecListExtendedElectrics {
                electrics_generator,
                electrics_starter,
                electrics_wipers_electrics,
                electrics_engine_fan,
                electrics_engine_fan_motor,
                electrics_window_lifter_drive,
            },
            wiring: CarSecListExtendedWiring {
                charger_insulation,
                battery_insulation,
                high_power_wiring,
            },
            fuel: CarSecListExtendedFuel { fuel_n_gas_leak },
        },
        description_table: CarSecListDescription {
            repair_required: CarSecListDescriptionRepair {
                appearance,
                interior,
                gloss,
                cleaning,
                wheels_status,
                wheels,
                tires_status,
                tires,
                window,
                additional_items,
                user_manual: "".to_owned(),
                emergency_stop_sign: "".to_owned(),
            },
        },
        opinions_table: CarSecListOpinions {
            performance_n_health_inspector,
            price_survey,
        },
        images,
    })
    // Ok(())
}
