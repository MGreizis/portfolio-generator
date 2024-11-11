import { useState } from 'react';
import { FaUser, FaBriefcase, FaEnvelope, FaHome, FaTimes } from 'react-icons/fa';

const Sidebar = () => {
  const [isOpen, setIsOpen] = useState(false);

  const navItems = [
    { label: 'Home', icon: <FaHome />, link: '/' },
    { label: 'About', icon: <FaUser />, link: '/about' },
    { label: 'Portfolio', icon: <FaBriefcase />, link: '/portfolio' },
    { label: 'Contact', icon: <FaEnvelope />, link: '/contact' },
  ];

  return (
    <div className="md:w-64">
      {/* Toggle button for small screens */}
      <button className="md:hidden p-4 text-white bg-bgblack focus:outline-none" onClick={() => setIsOpen(!isOpen)}>
        ☰
      </button>

      {/* Sidebar menu */}
      <aside className={`bg-bgblack text-white md:block ${isOpen ? 'block' : 'hidden'} md:h-auto h-full fixed inset-0 md:relative w-64 p-4 transition-transform duration-300`}>
        {/* Close button for mobile view */}
        <button className="md:hidden mb-4 text-white focus:outline-none" onClick={() => setIsOpen(false)}>
          <FaTimes className="text-2xl" />
        </button>

        <ul className="space-y-8">
          {navItems.map((item) => (
            <li key={item.label} className="flex items-center space-x-2">
              <span className="text-xl">{item.icon}</span>
              <a href={item.link} className="text-sm font-medium hover:text-gray-400">
                {item.label}
              </a>
            </li>
          ))}
        </ul>
      </aside>

      {/* Overlay to close sidebar when clicking outside */}
      {isOpen && <div className="fixed inset-0 bg-black opacity-50 md:hidden" onClick={() => setIsOpen(false)}></div>}
    </div>
  );
};

export default Sidebar;
