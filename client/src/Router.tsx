import {
  Route,
  createBrowserRouter,
  createRoutesFromElements,
  useNavigate,
} from "react-router-dom";
import App from "./App.tsx";
import { useEffect } from "react";

export const Router = createBrowserRouter(
  createRoutesFromElements(
    <Route path="/" element={<RedirectComponent />}>
      <Route path="/chatician" element={<App />} />
    </Route>
  )
);

function RedirectComponent() {
  const navigate = useNavigate();
  const searchParams = new URLSearchParams(window.location.search);
  searchParams.set("drawer", "open");
  console.log("Redirecting...");
  useEffect(() => {
    navigate(`/chatician/?${searchParams.toString()}`);
  }, []);
  return <App />;
}
