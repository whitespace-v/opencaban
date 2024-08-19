import Text from '@/app/Components/UIKIT/Text'
import classes from '../../styles/Pages/Home/HomeContentplates.module.scss'
import Image from 'next/image'
const HomeContentplates = () => {
    return (
        <div className={classes["HomeContentplates"]}>
            <div className={classes["HomeContentplates__title"]}>
                <Text fz='fzbig' fw='fwbig' color='white'>Почему CABAN ?</Text>
                <Text fz='fzmed'>
                    Мы каждый день пытаемся сделать наш сервис еще лучше и масштабнее.
                </Text>
            </div>
            <div className={classes["HomeContentplates__plates"]}>
                <div className={classes["HomeContentplates__plates-plate"]}>
                    <Image src='/Pages/Plates/supra.png'
                        height={0}
                        width={0}
                        alt='imageplaceholder'
                        unoptimized
                        className={classes["HomeContentplates__plates-plate-canvas"]}
                    />
                    <div className={classes["HomeContentplates__plates-plate-title"]}>
                        <Text color='white' fz='fzmed'>Все фотографии лота</Text>
                    </div>
                    <div className={classes["HomeContentplates__plates-plate-description"]}>
                        <Text>
                            Мы собираем все фотографии лотов, которые есть на аукционе, в оригинальном качестве.
                        </Text>
                    </div>
                </div>
                <div className={classes["HomeContentplates__plates-plate"]}>
                    <Image src='/Pages/Plates/supra.png'
                        height={0}
                        width={0}
                        alt='imageplaceholder'
                        unoptimized
                        className={classes["HomeContentplates__plates-plate-canvas"]}
                    />
                    <div className={classes["HomeContentplates__plates-plate-title"]}>
                        <Text color='white' fz='fzmed'>Удобство использования</Text>
                    </div>
                    <div className={classes["HomeContentplates__plates-plate-description"]}>
                        <Text>
                            Гибкость поиска и проработанный до мелочей интерфейс подарят вам приятные ощущения от взаимодействия.
                        </Text>
                    </div>
                </div>
                <div className={classes["HomeContentplates__plates-plate"]}>
                    <Image src='/Pages/Plates/supra.png'
                        height={0}
                        width={0}
                        alt='imageplaceholder'
                        unoptimized
                        className={classes["HomeContentplates__plates-plate-canvas"]}
                    />
                    <div className={classes["HomeContentplates__plates-plate-title"]}>
                        <Text color='white' fz='fzmed'>Скорость работы</Text>
                    </div>
                    <div className={classes["HomeContentplates__plates-plate-description"]}>
                        <Text>
                            Использование новейших мощных технологий позволяют нам добиться высочайшей скорости  работы сервиса.
                        </Text>
                    </div>
                </div>
                <div className={classes["HomeContentplates__plates-plate"]}>
                    <Image src='/Pages/Plates/supra.png'
                        height={0}
                        width={0}
                        alt='imageplaceholder'
                        unoptimized
                        className={classes["HomeContentplates__plates-plate-canvas"]}
                    />
                    <div className={classes["HomeContentplates__plates-plate-title"]}>
                        <Text color='white' fz='fzmed'>Большое количество источников</Text>
                    </div>
                    <div className={classes["HomeContentplates__plates-plate-description"]}>
                        <Text>
                            Мы собираем данные со всех возможных сервисов, у нас вы можете увидеть ротацию более 2млн машин в месяц.
                        </Text>
                    </div>
                </div>
            </div>
        </div >
    )
}

export default HomeContentplates