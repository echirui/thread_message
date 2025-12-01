import { BrowserRouter, Routes, Route } from 'react-router-dom';
import { MainLayout } from './layouts/MainLayout';

function App() {
  return (
    <BrowserRouter>
      <Routes>
        <Route path="/" element={<MainLayout />}>
          <Route index element={<div className="p-8 text-gray-600">Select a channel to start chatting</div>} />
        </Route>
      </Routes>
    </BrowserRouter>
  );
}

export default App;