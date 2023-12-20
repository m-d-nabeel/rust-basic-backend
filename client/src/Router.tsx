import {
  Route,
  createBrowserRouter,
  createRoutesFromElements,
} from "react-router-dom";
import App from "./App.tsx";

export const Router = createBrowserRouter(
  createRoutesFromElements(<Route path="/" element={<App />} />)
);
