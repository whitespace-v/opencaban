import { AppDispatch } from "../../store"
import { searchSlice } from "./SearchSlice"

export const loginLoading = (login: string) => async(dispatch: AppDispatch) => {
    try {
        dispatch(searchSlice.actions.loginLoading(login))
    } catch (e) {
        console.log(e)
    }
}