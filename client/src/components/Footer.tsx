import { Zap } from "lucide-react";

const Footer = () => {
  return (
    <footer className="w-full p-4 text-center">
      <p className="text-sm text-muted-foreground flex items-center justify-center gap-1">
        Built on Solana
        <Zap className="w-4 h-4 text-yellow-400" />
      </p>
    </footer>
  );
};

export default Footer;