'use client'
import { IoIosArrowDown } from 'react-icons/io'
import classes from '../../styles/Components/HOC/Float.module.scss'
import Button from '../UIKIT/Button'
import Text from '../UIKIT/Text'
import FloatSearch from './FloatSearch'
import Link from 'next/link'
import { useState } from 'react'

const Float = () => {
    const [state, setState] = useState<boolean>(false)
    return (
        <div className={classes["Float"]}>
            <div className={classes["Float__interaction"]}>
                <Link href={'/'}>
                    <Text color='white' fz='fzbig'>
                        CABAN
                    </Text>
                </Link>
                <div className={classes["Float__interaction_search"]}
                    onMouseEnter={() => setState(!state)}
                    onMouseLeave={() => setState(!state)}
                >
                    <Button type='box' color='gray' icon={<IoIosArrowDown />}>
                        Поиск
                    </Button>
                </div>
                <Button type='text' color='gray'>
                    Тарифы
                </Button>
                <Button type='text' color='gray'>
                    API
                </Button>
            </div>
            <div className={classes["Float__authentication"]}>
                <Button type='text' color='gray' to='authentication'>
                    Вход
                </Button>
                <Button type='box' color='gray'>
                    Регистрация
                </Button>
                <Button type='box' color='white'>
                    Связаться
                </Button>
            </div>
            <FloatSearch state={state} />
        </div>
    )
}

export default Float