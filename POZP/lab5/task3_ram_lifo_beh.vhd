----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date: 11/28/2021 06:45:58 PM
-- Design Name: 
-- Module Name: task3_ram_lifo_beh - Behavioral
-- Project Name: 
-- Target Devices: 
-- Tool Versions: 
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

-- Uncomment the following library declaration if using
-- arithmetic functions with Signed or Unsigned values
use IEEE.NUMERIC_STD.ALL;

-- Uncomment the following library declaration if instantiating
-- any Xilinx leaf cells in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity task3_ram_lifo_beh is
    generic (
        address: integer := 2;
        data: integer := 4
    );
    Port (
        CLK : in STD_LOGIC;
        PUSH: in STD_LOGIC;
        POP : in STD_LOGIC;
        Din : in STD_LOGIC_VECTOR(data - 1 downto 0);
		  Dout: out STD_LOGIC_VECTOR(data - 1 downto 0)
    );
end task3_ram_lifo_beh;

architecture Behavioral of task3_ram_lifo_beh is
    signal idx: integer range 0 to 2 ** address - 1;
    signal addr: STD_LOGIC_VECTOR(address - 1 downto 0);
    signal RW: STD_LOGIC;
    
begin
    main: process(CLK, PUSH, POP)
    begin
        if rising_edge(CLK) then
            if POP = '1' then
					idx <= idx - 1;
               RW <= '0';
            elsif PUSH = '1' then
					idx <= idx + 1;
               RW <= '1';
				else 
					RW <= '0';
            end if;
        end if;
    end process;
    
    addr <= STD_LOGIC_VECTOR(to_unsigned(idx, address));
    
    RAM: entity work.task3_ram_beh 
			generic map (address, data) 
         port map (CLK, RW, addr, Din, Dout);
end Behavioral;
