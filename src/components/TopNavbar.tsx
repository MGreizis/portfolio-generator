import { FaEye, FaSave, FaDownload } from 'react-icons/fa';

const TopNavbar = () => {
  return (
    <nav className="flex items-center justify-between bg-bgblack text-white p-4 w-full">
      <span className="flex" aria-label="Global">
        <a className="text-lg font-bold uppercase inline-block whitespace-nowrap" href="/">
          PGen
        </a>
      </span>

      <div className="space-x-4 hidden md:flex">
        <button className="flex items-center space-x-2 bg-gray-700 px-4 py-2 rounded-md">
          <FaEye />
          <span>Preview</span>
        </button>
        <button className="flex items-center space-x-2 bg-gray-700 px-4 py-2 rounded-md">
          <FaSave />
          <span>Save</span>
        </button>
        <button className="flex items-center space-x-2 bg-gray-700 px-4 py-2 rounded-md">
          <FaDownload />
          <span>Export</span>
        </button>
      </div>

      {/* Overflow menu for mobile screens */}
      <div className="md:hidden">
        <button className="p-4 text-white bg-gray-700 focus:outline-none">☰</button>
      </div>
    </nav>
  );
};

export default TopNavbar;
