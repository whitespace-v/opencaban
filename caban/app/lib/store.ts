import { combineReducers, configureStore } from '@reduxjs/toolkit';
import searchSlice from './features/search/SearchSlice';

const rootReducer = combineReducers({searchSlice})
export const makeStore = () => configureStore({reducer: rootReducer});
export type AppStore = ReturnType<typeof makeStore>;
export type RootState = ReturnType<AppStore['getState']>;
export type AppDispatch = AppStore['dispatch'];