'use client'
import classes from '../../styles/Components/HOC/FloatSearch.module.scss'
import { loginLoading } from "@/app/lib/features/search/SearchActions"
import { useAppDispatch, useAppSelector, useAppStore } from "@/app/lib/hooks"
import Text from "../UIKIT/Text"

const FloatSearch = ({ state }: { state: boolean }) => {
    // const store = useAppStore()
    const { loading } = useAppSelector(state => state.searchSlice)
    const dispatch = useAppDispatch()
    console.log(loading)
    return (
        <div
            className={state ? classes["FloatSearch"] + " " + classes["open"] : classes["FloatSearch"]}
            onClick={() => dispatch(loginLoading('asdasd'))}
        >
            <Text>
                {loading ? "yes" : 'asdf'}
            </Text>
        </div>
    )
}
export default FloatSearch