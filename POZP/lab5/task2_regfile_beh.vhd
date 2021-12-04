----------------------------------------------------------------------------------
-- Company: 
-- Engineer: 
-- 
-- Create Date: 11/28/2021 05:57:53 PM
-- Design Name: 
-- Module Name: task2_regfile_beh - Behavioral
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
use IEEE.std_logic_arith.ALL;

-- Uncomment the following library declaration if instantiating
-- any Xilinx leaf cells in this code.
--library UNISIM;
--use UNISIM.VComponents.all;

entity REG_FILE is
    generic(
        INITREG: STD_LOGIC_VECTOR := "0000";
        p: integer := 2
    );
    Port (
        INIT: in STD_LOGIC;
        WDP: in STD_LOGIC_VECTOR(INITREG'range);
        WA: in STD_LOGIC_VECTOR(p - 1 downto 0);
        WE: in STD_LOGIC;
        
        RA: in STD_LOGIC_VECTOR(p - 1 downto 0);
        
        RDP: out STD_LOGIC_VECTOR(INITREG'range)
    );
end REG_FILE;

architecture Behavioral of REG_FILE is
    signal wa_dec: STD_LOGIC_VECTOR(2 ** p - 1 downto 0);
    signal ra_dec: STD_LOGIC_VECTOR(2 ** p - 1 downto 0);
    signal rdp_t: STD_LOGIC_VECTOR(INITREG'range);
begin
    WAD: process(WA)
    begin
        for i in 0 to 2 ** p - 1 loop
            if i = CONV_INTEGER(unsigned(WA)) then
                wa_dec(i) <= '1';
            else
                wa_dec(i) <= '0';
            end if;
        end loop;
    end process;
    
    RAD: process(RA)
    begin
        for i in 0 to (2 ** p - 1) loop
            if i = CONV_INTEGER(unsigned(RA)) then
                ra_dec(i) <= '1';
            else
                ra_dec(i) <= '0';
            end if;
        end loop;
    end process;
    
    REG: for i in (2 ** p - 1) downto 0 generate
        REG: entity work.task2_reg_n_beh 
				generic map(bitness => INITREG'length) 
				port map(INIT, wa_dec(i), ra_dec(i), WE, WDP, rdp_t);
    end generate;
    RDP <= rdp_t;

end Behavioral;
