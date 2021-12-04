----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date: 11/28/2021 06:45:26 PM
-- Design Name: 
-- Module Name: tb_task3_ram - Behavioral
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
--use IEEE.NUMERIC_STD.ALL;

-- Uncomment the following library declaration if instantiating
-- any Xilinx leaf cells in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity tb_task3_ram is
--  Port ( );
end tb_task3_ram;

architecture Behavioral of tb_task3_ram is
    constant period: time := 50ns;
    constant address: integer := 2;
    constant data: integer := 4;

    signal CLK: STD_LOGIC := '0';
    signal RW: STD_LOGIC := '0';
    signal A: STD_LOGIC_VECTOR(address - 1 downto 0) := (others => '0');
    signal Din: STD_LOGIC_VECTOR(data - 1 downto 0);
	 signal Dout: STD_LOGIC_VECTOR(data - 1 downto 0);

    
begin
    uut_1: entity work.task3_ram_beh 
			generic map (address, data) 
			port map (CLK, RW, A, Din, Dout);
    
    CLK_proc: process
    begin
        CLK <= '0';
        wait for period / 2;
        CLK <= '1';
        wait for period / 2;
    end process;
    
    test_proc: process
    begin        
        RW <= '1';
        A <= "00";
        Din <= "1011";
        wait for period * 2;
        
        RW <= '0';
        A <= "00";
        wait for period * 2;
        
        RW <= '1';
        A <= "11";
        Din <= "1000";
        wait for period * 2;
         
        RW <= '0';
        A <= "00";
        wait for period * 2;
		  
        RW <= '0';
        A <= "11";
        wait for period * 2;
        RW <= '1';
        A <= "00";
        Din <= "1011";
        wait for period * 2;
        
        RW <= '0';
        A <= "00";
        wait for period * 2;
        
        RW <= '1';
        A <= "11";
        Din <= "1000";
        wait for period * 2;
         
        RW <= '0';
        A <= "00";
        wait for period * 2;
		  
        RW <= '0';
        A <= "11";
        wait for period * 2;
        wait;
    end process;
end Behavioral;
