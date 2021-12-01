----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date:    13:06:30 11/20/2021 
-- Design Name: 
-- Module Name:    SHIFT_REG_STR - Behavioral 
-- Project Name: 
-- Target Devices: 
-- Tool versions: 
-- Description: 
--
-- Dependencies: 
--
-- Revision: 
-- Revision 0.01 - File Created
-- Additional Comments: 
--
----------------------------------------------------------------------------------
library IEEE;
use IEEE.STD_LOGIC_1164.ALL;

library UNISIM;
use UNISIM.VComponents.ALL;

entity SHIFT_REG_STR is
    generic (N : integer := 4);
    port (RST : in std_logic;
          SE : in std_logic;
          Sin : in std_logic;
          Clk : in std_logic;
          POut : out std_logic_vector(N-1 downto 0));
end SHIFT_REG_STR;

architecture Behavioral of SHIFT_REG_STR is

signal ff_bus : std_logic_vector(N-1 downto 0);

begin
  ff1: FDCE port map (
		Q => ff_bus(0), 
		C => Clk, 
		CE => SE, 
		CLR => RST, 
		D => Sin
	);

  flipflops : for i in 1 to N-1 generate
        ffi: FDCE port map (
				Q => ff_bus(i),
				C => Clk,
				CE => SE,
				CLR => RST,
				D => ff_bus(i - 1)
			);
    end generate;

    POut <= ff_bus;

end Behavioral;