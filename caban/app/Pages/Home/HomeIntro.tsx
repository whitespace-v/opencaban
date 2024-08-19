import Text from '@/app/Components/UIKIT/Text'
import classes from '../../styles/Pages/Home/HomeIntro.module.scss'
import Button from '@/app/Components/UIKIT/Button'
import { IoIosArrowForward } from 'react-icons/io'
const HomeIntro = () => {
    return (
        <div className={classes["HomeIntro"]}>
            <div className={classes["HomeIntro__news"]}>
                <div className={classes["HomeIntro__news_hot"]}>
                    <Text color='cyan' fz='fzbig'>+145k авто/мес</Text>
                </div>
                <div className={classes["HomeIntro__news_feature"]}>
                    <Text>Добавили</Text>
                    <Text color='white'>24 марта</Text>
                    <Text>новый источник.</Text>
                </div>
                <div className={classes["HomeIntro__news_interaction"]}>
                    <Button type='round' color="white" icon={<IoIosArrowForward />}>
                        Наше дерево планов
                    </Button>
                </div>
            </div>
            <div className={classes["HomeIntro__shout"]}>
                <Text color='gradient' fz='fzhuge' fw='fwbig'>
                    Car Auction Buffer Access Network
                </Text>
            </div>
            <div className={classes["HomeIntro__greetings"]}>
                <Text fz="fzmed">
                    Мощный сервис по сбору информации и изображений с крупнейших аукционов по
                    всему миру, позволяющий вам подобрать
                </Text>
                <Text color='white' fz="fzmed">
                    лучший автомобиль
                </Text>
                <Text fz="fzmed">
                    из предложенных визуально удостовериться в надлежащем состоянии и
                    достаточной комплектакции автомобиля, а также преобретать автомобили
                    по самой выгодной цене.
                </Text>
            </div>
            <div className={classes["HomeIntro__interaction"]}>
                <div className={classes["HomeIntro__interaction_sight"]}>
                    <Button type='box' color="white">
                        К поиску
                    </Button>
                    <Button type='box' color="gray">
                        Авторизация
                    </Button>
                </div>
                <div className={classes["HomeIntro__interaction_api"]}>
                    <Text font='mono' fz='fzsmol'>
                        {"{~} caban-api для интеграции с вашим сайтом"}
                    </Text>
                </div>
            </div>

        </div>
    )
}

export default HomeIntro