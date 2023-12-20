import { cn } from "@/lib/utils";
import {
  MessageCircleIcon,
  MoreVerticalIcon,
  PhoneIcon,
  VideoIcon,
} from "lucide-react";
import { useEffect, useState } from "react";
import { useLocation, useNavigate } from "react-router-dom";

const menuItems = [
  { label: "Profile", option: "profile" },
  { label: "Select other user", option: "select-other-user" },
  { label: "Logout", option: "logout" },
];

function ChatHeader() {
  const [menuOpen, setMenuOpen] = useState(false);
  const location = useLocation();
  const navigate = useNavigate();

  useEffect(() => {
    const searchParams = new URLSearchParams(window.location.search);
    setMenuOpen(searchParams.get("tr_menu") === "true");
  }, [location]);

  const handleMenuClick = (
    e: React.MouseEvent<HTMLLIElement, MouseEvent>,
    option: string
  ) => {
    e.stopPropagation();
    const searchParams = new URLSearchParams(window.location.search);
    searchParams.set("menu_option", option);
    navigate(`?${searchParams.toString()}`);
  };

  const handleMenuToggle = () => {
    const searchParams = new URLSearchParams(window.location.search);
    searchParams.set("tr_menu", (!menuOpen).toString());
    navigate(`?${searchParams.toString()}`);
    setMenuOpen(!menuOpen);
  };

  const handleMenuClose = () => {
    if (menuOpen) {
      const searchParams = new URLSearchParams(window.location.search);
      if (searchParams.has("tr_menu")) {
        searchParams.delete("tr_menu");
        navigate(`?${searchParams.toString()}`);
      }
      setMenuOpen(false);
    }
  };

  return (
    <header
      className="flex items-center justify-between p-4 bg-gray-100"
      onClick={handleMenuClose}
    >
      <div className="flex items-center space-x-2">
        <MessageCircleIcon className="text-gray-600" />
        <h1 className="text-lg font-semibold">Santa Claus</h1>
      </div>
      <div className="flex items-center space-x-1">
        <PhoneIcon className="text-gray-600" />
        <VideoIcon className="text-gray-600" />
        <MoreVerticalIcon
          onClick={(e) => {
            e.stopPropagation();
            handleMenuToggle();
          }}
          className="text-gray-600"
        />
      </div>
      <div
        className={cn(
          "absolute right-1 top-16 w-[200px] bg-white shadow-lg rounded-lg overflow-hidden",
          !menuOpen && "hidden"
        )}
      >
        <ul className="divide-y divide-gray-200">
          {menuItems.map((item) => (
            <li
              key={item.option}
              onClick={(e) => handleMenuClick(e, item.option)}
              className="px-4 py-2 hover:bg-gray-100 cursor-pointer"
            >
              {item.label}
            </li>
          ))}
        </ul>
      </div>
    </header>
  );
}

export default ChatHeader;
