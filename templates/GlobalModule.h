////////////////////////////////////////////////////////////////////////////////
// /****************************************************************************
// **
// ** Copyright (C) 2015-2022 M-Way Solutions GmbH
// ** Contact: https://www.blureange.io/licensing
// **
// ** This file is part of the Bluerange/FruityMesh implementation
// **
// ** $BR_BEGIN_LICENSE:GPL-EXCEPT$
// ** Commercial License Usage
// ** Licensees holding valid commercial Bluerange licenses may use this file in
// ** accordance with the commercial license agreement provided with the
// ** Software or, alternatively, in accordance with the terms contained in
// ** a written agreement between them and M-Way Solutions GmbH.
// ** For licensing terms and conditions see https://www.bluerange.io/terms-conditions. For further
// ** information use the contact form at https://www.bluerange.io/contact.
// **
// ** GNU General Public License Usage
// ** Alternatively, this file may be used under the terms of the GNU
// ** General Public License version 3 as published by the Free Software
// ** Foundation with exceptions as appearing in the file LICENSE.GPL3-EXCEPT
// ** included in the packaging of this file. Please review the following
// ** information to ensure the GNU General Public License requirements will
// ** be met: https://www.gnu.org/licenses/gpl-3.0.html.
// **
// ** $BR_END_LICENSE$
// **
// ****************************************************************************/
////////////////////////////////////////////////////////////////////////////////

#pragma once

#include <Module.h>

constexpr VendorModuleId {{module_name upper}}_MODULE_ID = GET_VENDOR_MODULE_ID({{vendor_id}}, {{vendor_module_id}});
constexpr u8 {{module_name upper}}_MODULE_CONFIG_VERSION = 1;

/*
 * {{module_name}} module description:
 * {{module_description}}
 */
class {{module_name}Module: public Module
{
    private:

        //Module configuration that is saved persistently (size must be multiple of 4)
        struct {{module_name}}ModuleConfiguration : ModuleConfiguration{
            //Insert more persistent config values here
        };

        {{module_name}}ModuleConfiguration configuration;

        enum {{module_name}}ModuleTriggerActionMessages{
            TRIGGER_{{module_name upper}}=0
        };

        enum {{module_name}}ModuleActionResponseMessages{
            {{module_name upper}}_RESPONSE=0
        };

    public:
        {{module_name}}Module();

        void ConfigurationLoadedHandler(u8* migratableConfig, u16 migratableConfigLength) override;

        void ResetToDefaultConfiguration() override;

        void TimerEventHandler(u16 passedTimeDs) override;

        void MeshMessageReceivedHandler(BaseConnection* connection, BaseConnectionSendData* sendData, ConnPacketHeader const * packetHeader) override;

        #ifdef TERMINAL_ENABLED
        TerminalCommandHandlerReturnType TerminalCommandHandler(const char* commandArgs[], u8 commandArgsSize) override;
        #endif
};
