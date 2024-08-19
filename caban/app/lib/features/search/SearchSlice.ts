import { PayloadAction, createSlice } from "@reduxjs/toolkit";

interface SearchState {
    loading: boolean;
}

const initialState: SearchState = {
    loading: false
}
export const searchSlice = createSlice({
    name: 'search',
    initialState,
    reducers: {
        loginLoading(state, action: PayloadAction<string>) {
            state.loading = true
        },
    }
})

export default searchSlice.reducer