import { Link, useNavigate } from "react-router-dom";
import { useAuth } from "../AuthProvider";

export default function AdminSidebar() {
  const { logout } = useAuth();
  const navigate = useNavigate();

  return (
    <aside className="w-[300px] shrink-0 border-l border-gray-300 p-6 bg-white">
      <h2 className="text-2xl font-semibold mb-4">Admin Pages</h2>
      <nav className="flex flex-col space-y-2">
        <Link to="/operators" className="text-blue-600 hover:underline">
          Operators
        </Link>
        <span className="text-gray-500 cursor-not-allowed">Lorem</span>
        <span className="text-gray-500 cursor-not-allowed">Ipsum</span>
        <span className="text-gray-500 cursor-not-allowed">Dolor</span>
      </nav>
      <button
        onClick={() => {
          logout();
          navigate("/");
        }}
        className="mt-6 text-red-600 hover:underline text-left"
      >
        Sign Out
      </button>
    </aside>
  );
}
